use bevy_egui::{
    egui::{style::Margin, Color32, FontFamily::Monospace, FontId, Frame, RichText, SidePanel},
    EguiContext,
};
use bevy_rapier3d::prelude::{Collider, CollisionEvent, QueryFilter, RapierContext};
use bevy_renet::{
    renet::{ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig},
    run_if_client_connected, RenetClientPlugin,
};
use game::*;
use std::{net::UdpSocket, time::SystemTime};

pub mod plugins;

const FRAME_FILL: Color32 = Color32::TRANSPARENT;
const TEXT_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 255, 0, 100);

pub fn send_messages_to_server(
    mut to_server_events: EventReader<events::ToServer>,
    mut client: ResMut<RenetClient>,
) {
    for message in to_server_events.iter() {
        client.send_message(
            DefaultChannel::Reliable,
            bincode::serialize(message).unwrap(),
        );
    }
}

/// That is, "process 'to-client' events"
/// definitely NOT "process to 'client events'"
pub fn process_to_client_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_state: ResMut<State<resources::GameState>>,
    mut to_client_events: EventReader<events::ToClient>,
    mut to_server_events: EventWriter<events::ToServer>,
    client: Res<RenetClient>,
) {
    let my_id = client.client_id();
    for message in to_client_events.iter() {
        trace!("Message for {my_id}: {message:?}");
        match message {
            events::ToClient::SetGameState(new_game_state) => {
                debug!("  got `SetGameState`. Setting state to {new_game_state:?}");
                let _ = game_state.overwrite_set(*new_game_state);
            }
            events::ToClient::InhabitantRotation { .. } => {
                // handled by separate system
            }
            events::ToClient::SetGameConfig(game_config) => {
                let inhabited_mass_id = *game_config.client_mass_map.get(&my_id).unwrap();
                resources::init_masses(
                    inhabited_mass_id,
                    game_config.init_data.clone(),
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );
                commands.insert_resource(game_config.clone());
                let message = events::ToServer::Ready;
                debug!("  enqueuing message for server `{message:?}`");
                to_server_events.send(message);
            }
            events::ToClient::ProjectileFired(_) => {
                // not handled here
            }
        }
    }
}

pub fn receive_messages_from_server(
    mut client: ResMut<RenetClient>,
    mut to_client_events: EventWriter<events::ToClient>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        to_client_events.send(bincode::deserialize(&message).unwrap());
    }
}

pub fn new_renet_client(client_id: u64, address: String) -> RenetClient {
    debug!("I AM client {client_id:?}");
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
        user_data: None,
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

pub fn control(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut to_server_events: EventWriter<events::ToServer>,
    mut inhabitant_query: Query<&mut Transform, With<components::ClientInhabited>>,
) {
    let nudge = TAU / 10000.0;
    let keys_scaling = 10.0;

    // rotation about local axes
    let mut rotation = Vec3::ZERO;

    // FIXME: What's the general way of handling this?
    // How to generally get "with modifiers"?
    if !keys.pressed(KeyCode::RShift) {
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
            let message = events::ToServer::Rotation(transform.rotation);
            to_server_events.send(message);
        } else {
            error!("ClientInhabited entity not present");
        }
    }
}

/// This does not include the client's mass
pub fn rotate_inhabitable_masses(
    mut to_client_events: EventReader<events::ToClient>,
    mut inhabitable_masses: Query<
        (&mut Transform, &components::MassID),
        With<components::Inhabitable>,
    >,
    game_config: Res<resources::GameConfig>,
) {
    for message in to_client_events.iter() {
        if let events::ToClient::InhabitantRotation {
            client_id,
            rotation,
        } = message
        {
            trace!("  got `InhabitantRotation`. Rotating mass {client_id}");
            let inhabited_mass_id = *game_config.client_mass_map.get(client_id).unwrap();
            for (mut mass_transform, &components::MassID(mass_id)) in inhabitable_masses.iter_mut()
            {
                // The choices seem to be
                //   1. Loop through masses and look for a match, like we do here.
                //   2. Maintain a mass_id <-> mass_entity mapping.
                // The latter was tried (see history), but the complexity cost doesn't
                // seem to be worth the "performance gain" (of which there may be little.)
                // It might be worth swapping the loops (iter masses then messages.)
                if inhabited_mass_id == mass_id {
                    mass_transform.rotation = *rotation;
                    break;
                }
            }
        }
    }
}

pub fn client_waiting_screen(
    mut ctx: ResMut<EguiContext>,
    game_config: Res<resources::GameConfig>,
) {
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
                RichText::new("Waiting for more players\n\nConnected:")
                    .color(TEXT_COLOR)
                    .font(FontId {
                        size: 20.0,
                        family: Monospace,
                    }),
            );
            ui.separator();
            for (&id, _) in game_config.client_mass_map.iter() {
                let nick = to_nick(id);
                let text = format!("{nick}");
                ui.label(RichText::new(text).color(TEXT_COLOR).font(FontId {
                    size: 16.0,
                    family: Monospace,
                }));
            }
        });
}

pub fn handle_projectile_engagement(
    mass_query: Query<
        (&Transform, &components::MassID),
        (
            Without<components::Inhabitable>,
            Without<components::ClientInhabited>,
        ),
    >,
    inhabited_mass_query: Query<
        (&Transform, &components::MassID),
        With<components::ClientInhabited>,
    >,
    rapier_context: Res<RapierContext>,
    mut to_server_events: EventWriter<events::ToServer>,
    mut sights_query: Query<&mut Visibility, With<components::Sights>>,
    keys: Res<Input<KeyCode>>,
) {
    if let Ok((client_pov, &components::MassID(from_mass_id))) = inhabited_mass_query.get_single() {
        let ray_origin = client_pov.translation;
        let ray_direction = -client_pov.local_z();
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
                    to_server_events.send(events::ToServer::ProjectileFired(
                        events::ProjectileFlight {
                            launch_time,
                            from_mass_id,
                            to_mass_id,
                            local_impact_direction,
                        },
                    ));
                }
            } else {
                // NOTE: This happens because `QueryFilter` in `rapier_context.cast_ray` is
                // `only_dynamic()`, which includes _other_ inhabited masses. Another thing
                // fixed by a different `QueryFilter` in the above (I think).
            }
        } else {
            for mut visibility in sights_query.iter_mut() {
                visibility.is_visible = false;
            }
        }
    } else {
        error!("No client-inhabited mass");
    }
}

pub fn handle_projectile_fired(
    mut to_client_events: EventReader<events::ToClient>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for message in to_client_events.iter() {
        if let events::ToClient::ProjectileFired(projectile_flight) = message {
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
    masses_query: Query<(&Transform, &components::MassID), Without<events::ProjectileFlight>>,
) {
    let proportion_of = 1.0 / 512.0;
    let portions_per_second = 128.0 * 3.0;

    for (projectile_id, mut projectile_transform, mut projectile_visibility, projectile_flight) in
        projectile_query.iter_mut()
    {
        projectile_visibility.is_visible = true;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let seconds_elapsed = (now - projectile_flight.launch_time) as f32 / 1_000.0;
        // FIXME: This could be collapsed into something sexier, `for_each().fold()...`
        // Something like that.
        let mut from_transform = None;
        let mut to_transform = None;
        for (transform, &components::MassID(mass_id)) in masses_query.iter() {
            if projectile_flight.from_mass_id == mass_id {
                from_transform = Some(transform);
            }
            if projectile_flight.to_mass_id == mass_id {
                to_transform = Some(transform);
            }
        }
        if from_transform.is_none() {
            panic!("The transform FROM which projectile {projectile_id:?} originated (an inhabited mass) has disappeared!");
        }
        if to_transform.is_none() {
            // FIXME: When a minor mass gets merged into a major, what should happen to in-flight projectiles
            // that were targeting that mass? What if the major mass is an inhabited mass??
            warn!("The transform TO which projectile {projectile_id:?} as headed (the target mass) has disappeared. Despawning projectile");
            commands.entity(projectile_id).despawn_recursive();
            continue;
        }
        let from_transform = from_transform.unwrap();
        let to_transform = to_transform.unwrap();

        // The impact site/taget is the _surface of_ the mass
        let impact_site = to_transform.translation
            + projectile_flight.local_impact_direction * scale_to_radius(to_transform.scale);
        let flight_vector = impact_site - from_transform.translation;
        let flight_progress = flight_vector * proportion_of * portions_per_second * seconds_elapsed;
        projectile_transform.translation = from_transform.translation + flight_progress;
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
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<&events::ProjectileFlight>,
    mass_query: Query<(
        With<components::MassID>,
        Without<components::ClientInhabited>,
        Without<components::Inhabitable>,
    )>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e0, e1, _) = collision_event {
            let e0_is_projectile = projectile_query.contains(*e0);
            let e1_is_projectile = projectile_query.contains(*e1);
            if e0_is_projectile ^ e1_is_projectile {
                let projectile_id = if e0_is_projectile { e0 } else { e1 };
                let projectile_flight = projectile_query.get(*projectile_id).unwrap();
                let mass_id = if !e0_is_projectile { e0 } else { e1 };
                if mass_query.contains(*mass_id) {
                    // we always have unit diameter and _scale_, so a "unit vector" will
                    // exactly end at the "surface" in the mass's transform.
                    let local_impact_site = projectile_flight.local_impact_direction;
                    trace!(
                        "Collider {projectile_id:?} has collided with uninhabited mass {mass_id:?}. Spawning explosion animation."
                    );
                    commands.entity(*mass_id).with_children(|child| {
                        child
                            .spawn(PbrBundle {
                                transform: Transform::from_translation(local_impact_site),
                                mesh: meshes.add(Mesh::from(shape::Icosphere {
                                    radius: 0.5,
                                    ..Default::default()
                                })),
                                material: materials.add(Color::rgb_u8(255, 255, 255).into()),
                                ..Default::default()
                            })
                            .insert(components::Explosion {
                                timer: Timer::from_seconds(5.0, TimerMode::Once),
                            });
                    });
                    trace!("Despawning collided projectile {projectile_id:?}");
                    commands.entity(*projectile_id).despawn_recursive();
                }
            }
        }
    }
}

pub fn animate_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut explosions: Query<(Entity, &mut Transform, &mut components::Explosion)>,
) {
    for (explosion_id, mut transform, mut explosion) in explosions.iter_mut() {
        explosion.timer.tick(time.delta());
        if explosion.timer.finished() {
            trace!("Despawning completed explosion animation {explosion_id:?}");
            commands.entity(explosion_id).despawn_recursive();
        } else {
            let percent = explosion.timer.percent();
            let scale = 1.0 - percent;
            transform.scale = scale * Vec3::ONE;
        }
    }
}
