use crate::*;
use bevy_egui::{
    egui::{style::Margin, Color32, FontFamily::Monospace, FontId, Frame, RichText, SidePanel},
    EguiContext,
};
use bevy_rapier3d::prelude::{Collider, CollisionEvent, QueryFilter, RapierContext, RigidBody};
use bevy_renet::{
    renet::{ClientAuthentication, RenetClient, RenetConnectionConfig},
    run_if_client_connected, RenetClientPlugin,
};
use std::{net::UdpSocket, time::SystemTime};

const FRAME_FILL: Color32 = Color32::TRANSPARENT;
const TEXT_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 255, 0, 100);

pub fn send_messages_to_server(
    mut client_messages: EventReader<events::ClientMessage>,
    mut client: ResMut<RenetClient>,
) {
    for message in client_messages.iter() {
        client.send_message(CHANNEL_RELIABLE, bincode::serialize(message).unwrap());
    }
}

pub fn process_server_messages(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_state: ResMut<State<resources::GameState>>,
    mut mass_to_entity_map: ResMut<resources::MassIDToEntity>,
    mut server_messages: EventReader<events::ServerMessage>,
    mut client_messages: EventWriter<events::ClientMessage>,
    mut lobby: ResMut<resources::Lobby>,
    client: Res<RenetClient>,
) {
    let my_id = client.client_id();
    for message in server_messages.iter() {
        debug!("Message for {my_id}");
        match message {
            events::ServerMessage::Init(init_data) => {
                debug!("  got `Init`. Initializing with data receveid from server: {init_data:?}");
                // FIXME: so much clone
                *mass_to_entity_map = init_data
                    .clone()
                    .init(&mut commands, &mut meshes, &mut materials)
                    .clone();
                let message = events::ClientMessage::Ready;
                debug!("  enqueuing message for server `{message:?}`");
                client_messages.send(message);
            }
            events::ServerMessage::SetGameState(new_game_state) => {
                debug!("  got `SetGameState`. Setting state to {new_game_state:?}");
                let _ = game_state.overwrite_set(*new_game_state);
            }
            events::ServerMessage::SetPhysicsConfig(physics_config) => {
                debug!("  got `SetPhysicsConfig`. Inserting resource received from server: {physics_config:?}");
                commands.insert_resource(*physics_config);
            }
            events::ServerMessage::ClientRotation { .. } => {
                // handled by separate system
            }
            events::ServerMessage::ClientJoined { id, client_data } => {
                debug!("  got `ClientJoined`. Inserting entry for client {id}");
                if let Some(old) = lobby.clients.insert(*id, *client_data) {
                    warn!("  the value {old:?} was replaced for client {id}");
                }
                if *id == client.client_id() {
                    let inhabited_mass = mass_to_entity_map
                        .0
                        .get(&client_data.inhabited_mass_id)
                        .unwrap();
                    debug!("    server has assigned to me mass id {} which I map to entity {inhabited_mass:?}",
			   client_data.inhabited_mass_id);
                    let mut inhabited_mass_commands = commands.entity(*inhabited_mass);
                    debug!("    inserting `ClientInhabited` component into this mass entity (meaing 'this is mine')");
                    inhabited_mass_commands.insert(components::ClientInhabited);
                    inhabited_mass_commands.remove::<components::Inhabitable>();
                    // FIXME -- Figure out rapier `QueryFilter` so we don't need this (or do we?)
                    inhabited_mass_commands.remove::<RigidBody>();
                    inhabited_mass_commands.despawn_descendants();
                    debug!("    appending camera to inhabited mass to this entity");
                    inhabited_mass_commands.with_children(|child| {
                        child.spawn(Camera3dBundle::default());
                        debug!("    adding \"sights\"");
                        // FIXME -- this is so klunky
                        child
                            .spawn(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Icosphere {
                                    radius: 0.0005,
                                    ..Default::default()
                                })),
                                material: materials.add(Color::WHITE.into()),
                                transform: Transform::from_xyz(0.0, 0.0, -0.2),
                                visibility: Visibility::INVISIBLE,
                                ..Default::default()
                            })
                            .insert(components::Sights);
                        child
                            .spawn(PointLightBundle {
                                transform: Transform::from_xyz(0.0, 0.0, -0.15),
                                visibility: Visibility::INVISIBLE,
                                point_light: PointLight {
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(components::Sights);
                    });
                }
                debug!("    we now have lobby {lobby:?}");
            }
            events::ServerMessage::ProjectileFired(_) => {
                // not handled here
            }
        }
    }
}

pub fn receive_messages_from_server(
    mut client: ResMut<RenetClient>,
    mut server_messages: EventWriter<events::ServerMessage>,
) {
    while let Some(message) = client.receive_message(CHANNEL_RELIABLE) {
        server_messages.send(bincode::deserialize(&message).unwrap());
    }
}

pub fn new_renet_client(
    client_id: u64,
    client_preferences: resources::ClientPreferences,
    address: String,
) -> RenetClient {
    let address = if let Ok(address) = format!("{address}").parse() {
        address
    } else {
        panic!("Cannot parse address `{address}`");
    };
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr: address,
        user_data: Some(client_preferences.to_netcode_user_data()),
    };
    RenetClient::new(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        socket,
        RenetConnectionConfig::default(),
        authentication,
    )
    .unwrap()
}

pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Core);
        app.insert_resource(resources::Lobby::default());
        app.add_plugin(Spacetime);
        app.add_system_set(
            SystemSet::on_update(resources::GameState::Waiting)
                .with_system(client::client_waiting_screen),
        );
        app.add_plugin(RenetClientPlugin::default());

        app.add_system(client::send_messages_to_server.with_run_criteria(run_if_client_connected));
        app.add_system(client::process_server_messages.with_run_criteria(run_if_client_connected));
        app.add_system(
            client::receive_messages_from_server.with_run_criteria(run_if_client_connected),
        );
        app.add_system(panic_on_renet_error);
    }
}

pub fn control(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut client_messages: EventWriter<events::ClientMessage>,
    mut inhabitant_query: Query<&mut Transform, With<components::ClientInhabited>>,
) {
    let nudge = TAU / 10000.0;
    let keys_scaling = 10.0;

    // rotation about local axes
    let mut rotation = Vec3::ZERO;

    // IDEAR: we could just get key counts as f32 and multiply by nudge.
    //   A -> [0, 0, 1]
    //   D -> [0, 0, -1]
    // ...etc
    for key in keys.get_pressed() {
        match key {
            // pitch
            KeyCode::W => {
                rotation.x += nudge;
            }
            KeyCode::S => {
                rotation.x -= nudge;
            }
            // yaw
            KeyCode::A => {
                rotation.y += nudge;
            }
            KeyCode::D => {
                rotation.y -= nudge;
            }
            // roll
            KeyCode::Z => {
                rotation.z -= nudge;
            }
            KeyCode::X => {
                rotation.z += nudge;
            }
            _ => (),
        }
    }

    if rotation.length() > 0.0000001 {
        if let Ok(mut transform) = inhabitant_query.get_single_mut() {
            let frame_time = time.delta_seconds() * 60.0;
            rotation *= keys_scaling * frame_time;
            let local_x = transform.local_x();
            let local_y = transform.local_y();
            let local_z = transform.local_z();
            transform.rotate(Quat::from_axis_angle(local_x, rotation.x));
            transform.rotate(Quat::from_axis_angle(local_z, rotation.z));
            transform.rotate(Quat::from_axis_angle(local_y, rotation.y));
            let message = events::ClientMessage::Rotation(transform.rotation);
            client_messages.send(message);
        } else {
            error!("ClientInhabited entity not present");
        }
    }
}

/// This does not include the client's mass
pub fn rotate_inhabitable_masses(
    mut server_messages: EventReader<events::ServerMessage>,
    mut inhabitable_masses: Query<&mut Transform, With<components::Inhabitable>>,
    mass_to_entity_map: Res<resources::MassIDToEntity>,
    lobby: Res<resources::Lobby>,
) {
    for message in server_messages.iter() {
        if let events::ServerMessage::ClientRotation { id, rotation } = message {
            debug!("  got `ClientRotation`. Rotating mass {id}");
            let mass_id = lobby.clients.get(id).unwrap().inhabited_mass_id;
            if let Some(entity) = mass_to_entity_map.0.get(&mass_id) {
                if let Ok(mut mass_transform) = inhabitable_masses.get_mut(*entity) {
                    debug!("    found corresponding entity {entity:?}");
                    mass_transform.rotation = *rotation;
                } else {
                    error!(
                        "Entity map for mass ID {id} as entity {entity:?} which does not exist."
                    );
                }
            } else {
                error!(
                    "Unable to find client {id} in entity mapping {:?}",
                    mass_to_entity_map.0
                )
            }
        }
    }
}

pub fn client_waiting_screen(mut ctx: ResMut<EguiContext>, lobby: Res<resources::Lobby>) {
    SidePanel::left("client_waiting_screen")
        .resizable(false)
        .min_width(250.0)
        .frame(Frame {
            outer_margin: Margin::symmetric(10.0, 20.0),
            fill: FRAME_FILL,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.label(
                RichText::new("Waiting for players...")
                    .color(TEXT_COLOR)
                    .font(FontId {
                        size: 20.0,
                        family: Monospace,
                    }),
            );
            ui.separator();
            for (&id, &client_data) in lobby.clients.iter() {
                let nick = to_nick(id);
                let autostart = if client_data.preferences.autostart {
                    "autostart"
                } else {
                    "wait"
                };
                let text = format!("{nick}>  {autostart}");
                ui.label(RichText::new(text).color(TEXT_COLOR).font(FontId {
                    size: 16.0,
                    family: Monospace,
                }));
            }
        });
}

pub fn handle_projectile_engagement(
    mass_query: Query<(&Transform, &components::MassID), Without<components::Inhabitable>>,
    inhabited_mass_query: Query<
        (&Transform, &components::MassID),
        With<components::ClientInhabited>,
    >,
    rapier_context: Res<RapierContext>,
    mut client_messages: EventWriter<events::ClientMessage>,
    mut sights_query: Query<&mut Visibility, With<components::Sights>>,
    keys: Res<Input<KeyCode>>,
) {
    for (client_pov, &components::MassID(from_mass_id)) in inhabited_mass_query.iter() {
        let ray_origin = client_pov.translation;
        let ray_direction = -1.0 * client_pov.local_z();
        let intersection = rapier_context.cast_ray(
            ray_origin,
            ray_direction,
            150.0,
            false,
            QueryFilter::only_dynamic(),
        );
        if let Some((mass, distance)) = intersection {
            if let Ok((mass_transform, &components::MassID(to_mass_id))) = mass_query.get(mass) {
                for mut visibility in sights_query.iter_mut() {
                    visibility.is_visible = true;
                }
                if keys.just_pressed(KeyCode::Space) {
                    let global_impact_site = ray_origin + (ray_direction * distance);
                    let local_impact_direction =
                        (global_impact_site - mass_transform.translation).normalize();
                    let launch_time = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();
                    let current_direction = Some(-client_pov.local_z());
                    client_messages.send(events::ClientMessage::ProjectileFired(
                        events::ProjectileFlight {
                            launch_time,
                            from_mass_id,
                            to_mass_id,
                            local_impact_direction,
                            current_direction,
                        },
                    ));
                }
            } else {
                for mut visibility in sights_query.iter_mut() {
                    visibility.is_visible = false;
                }
            }
        }
    }
}

pub fn handle_projectile_fired(
    mut client_messages: EventReader<events::ServerMessage>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for message in client_messages.iter() {
        if let events::ServerMessage::ProjectileFired(projectile_flight) = message {
            let radius = 0.5;
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius,
                        ..Default::default()
                    })),
                    visibility: Visibility::INVISIBLE,
                    material: materials.add(StandardMaterial {
                        base_color: Color::RED + Color::WHITE * 0.2,
                        emissive: Color::rgb_u8(125, 125, 125),
                        unlit: true,
                        ..default()
                    }),
                    transform: Transform::from_scale(Vec3::ONE * radius),
                    ..default()
                })
                .insert(Collider::default())
                .insert(*projectile_flight)
                .with_children(|children| {
                    children.spawn(PointLightBundle {
                        point_light: PointLight {
                            intensity: 100.0,
                            color: Color::RED,
                            ..default()
                        },
                        ..default()
                    });
                });
        }
    }
}

pub fn move_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(
        Entity,
        &mut Transform,
        &mut Visibility,
        &events::ProjectileFlight,
    )>,
    masses_query: Query<
        (&Transform, &components::Momentum),
        (With<components::MassID>, Without<events::ProjectileFlight>),
    >,
    mass_to_entity_map: Res<resources::MassIDToEntity>,
) {
    let proportion_of = 1.0 / 512.0;
    let portions_per_second = 128.0 * 3.0;

    for (projectile_id, mut projectile_transform, mut projectile_visibility, projectile_flight) in
        projectile_query.iter_mut()
    {
        projectile_visibility.is_visible = true; // FIXME -- we need a better "way"
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let seconds_elapsed = (now - projectile_flight.launch_time) as f32 / 1_000.0;
        match mass_to_entity_map
            .get_entities([projectile_flight.from_mass_id, projectile_flight.to_mass_id])
        {
            Result::Ok([from_entity, to_entity]) => {
                match masses_query.get_many([from_entity, to_entity]) {
                    Ok(
                        [(from_transform, _), (to_transform, &components::Momentum { mass, .. })],
                    ) => {
                        // The impact site/taget is the _surface of_ the mass
                        let impact_site = to_transform.translation
                            + (projectile_flight.local_impact_direction
                                * mass_to_radius(mass)
                                * to_transform.scale.length()
                                / SQRT_3); // mysterious
                        let flight_vector = impact_site - from_transform.translation;
                        let flight_progress =
                            flight_vector * proportion_of * portions_per_second * seconds_elapsed;
                        projectile_transform.translation =
                            from_transform.translation + flight_progress;
                    }
                    Err(err) => {
                        error!("While getting projectile to/from: {err}");
                        debug!("Despawning projectile {projectile_id:?}");
                        commands.entity(projectile_id).despawn_recursive();
                    }
                }
            }
            Result::Err(err) => {
                error!("While trying to move projectile: {err}");
            }
        }
    }
}

pub fn set_window_title(mut windows: ResMut<Windows>, client: Res<RenetClient>) {
    let title = "Mass Gathering";
    let id = client.client_id();
    let nickname = to_nick(id).trim_end().to_string();
    let title = format!("{title} | nick: \"{nickname}\"");
    windows.primary_mut().set_title(title);
}

pub fn handle_projectile_collision(
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<Entity, With<events::ProjectileFlight>>,
    mass_query: Query<
        &Transform,
        (
            With<components::MassID>,
            Without<components::ClientInhabited>,
            Without<components::Inhabitable>,
        ),
    >,
) {
    for collision_event in collision_events.iter() {
        warn!("event");
        if let CollisionEvent::Started(e0, e1, flags) = collision_event {
            warn!("started");
            let e0_is_projectile = projectile_query.contains(*e0);
            let e1_is_projectile = projectile_query.contains(*e1);
            if e0_is_projectile ^ e1_is_projectile {
                warn!("and one was a projectile");
                let projectile_id = if e0_is_projectile { e0 } else { e1 };
                let mass_id = if !e0_is_projectile { e0 } else { e1 };
                if let Ok(mass_transform) = mass_query.get(*mass_id) {
                    warn!("yay, other was a mass");
                }
            } else {
                warn!("xor failed");
            }
        } else {
            warn!("not started");
        }
    }
}
