/// The client module
///
/// "Client" is currently defined as: the networking, gui (windows), and simulation parts put together to play the game, on a network.
/// Contrast "server", is a networking server, which requires a full simulation, and also can _optionally_ do the graphical stuff or
/// be headless.
use bevy_rapier3d::prelude::{QueryFilter, RapierContext};
use bevy_renet::{
    renet::{ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig},
    RenetClientPlugin,
};
use clap::Parser;
use game::simulation::FromSimulation;
use game::*;
use std::{net::UdpSocket, time::SystemTime};

pub mod plugins;

#[derive(Parser, Resource)]
pub struct ClientCliArgs {
    #[arg(long)]
    pub nickname: String,
    #[arg(long, default_value_t = format!("{SERVER_IP}:{SERVER_PORT}"))]
    pub address: String,
}

/// Unconditionally set `GameState` for each `SetGameState` event read
///
/// NOTE: This could be done on the server as well and we could forget
/// about the server writing its own state separately. In that case, is
/// ToClient a reasonable name for the event? (probably still yes)
///
/// refactor_tags: to_client_read, game_state_write
pub fn handle_set_game_state(
    mut game_state: ResMut<State<resources::GameState>>,
    mut to_client_events: EventReader<events::ToClient>,
) {
    for message in to_client_events.iter() {
        if let events::ToClient::SetGameState(state) = message {
            debug!("Setting state to {state:?}");
            let _ = game_state.overwrite_set(*state);
        }
    }
}

/// Insert `GameConfig` for each `SetGameConfig` event read
///
/// NOTE: Could we insert this on the server too a la `GameState`??
///
/// refactor_tags: to_client_read, commands, game_config_insert
pub fn handle_set_game_config(
    mut commands: Commands,
    mut to_client_events: EventReader<events::ToClient>,
) {
    for message in to_client_events.iter() {
        if let events::ToClient::SetGameConfig(game_config) = message {
            commands.insert_resource(game_config.clone());
        }
    }
}

/// Send every `ToServer` event read, over the network to the server
///
/// NOTE: Definitely only on "client"
///
/// refactor_tags: to_server_read, network_client_write
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

/// For every `ToClient` received over the network from the server, send it as an event
///
/// NOTE: Only "client" but, the server lacks this analog for its client-on-server.
///
/// refactor_tags: network_client_write, to_client_write
pub fn receive_messages_from_server(
    mut client: ResMut<RenetClient>,
    mut to_client_events: EventWriter<events::ToClient>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        to_client_events.send(bincode::deserialize(&message).unwrap());
    }
}

use bevy_egui::{
    egui::{style::Margin, Color32, FontFamily::Monospace, FontId, Frame, RichText, SidePanel},
    EguiContext,
};

/// Read various resources and display a toggleable info screen
///
/// NOTE: This system can tolerate absence of `GameConfig`, since this is available before its insertion.
///
/// refactor_tags: ui_state_read, game_state_read, game_config_read, cameras_read, network_client_read, egui, gui
pub fn info_text(
    mut ctx: ResMut<EguiContext>,
    ui_state: Res<resources::UiState>,
    game_state: Res<State<resources::GameState>>,
    game_config: Option<Res<resources::GameConfig>>,
    cameras: Query<(&Camera, &resources::CameraTag)>,
    client: Res<RenetClient>,
) {
    if !ui_state.show_info {
        return;
    }
    let my_id = client.client_id();
    let text_color = Color32::from_rgba_premultiplied(0, 255, 0, 100);
    SidePanel::left("info")
        .resizable(false)
        .min_width(250.0)
        .frame(Frame {
            outer_margin: Margin::symmetric(10.0, 20.0),
            fill: Color32::TRANSPARENT,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.label(
                RichText::new("`i` key toggles this [i]nfo menu")
                    .color(text_color)
                    .font(FontId {
                        size: 8.0,
                        family: Monospace,
                    }),
            );
            ui.label(
                RichText::new("`o` key swaps [o]bjective and client cameras")
                    .color(text_color)
                    .font(FontId {
                        size: 8.0,
                        family: Monospace,
                    }),
            );
            ui.separator();
            for (camera, tag) in cameras.iter() {
                if *tag == ui_state.camera {
                    if !camera.is_active {
                        warn!("Selected camera \"{}\" is not active!", ui_state.camera);
                    }
                    let line = format!("camera: {tag}");
                    ui.label(RichText::new(line).color(text_color).font(FontId {
                        size: 8.0,
                        family: Monospace,
                    }));
                }
            }
            let connected = if client.is_connected() { "yes" } else { "no" };
            let line = format!("connected: {connected}");
            ui.label(RichText::new(line).color(text_color).font(FontId {
                size: 8.0,
                family: Monospace,
            }));
            let game_state = game_state.current();
            let line = format!("game state: {game_state}");
            ui.label(RichText::new(line).color(text_color).font(FontId {
                size: 8.0,
                family: Monospace,
            }));
            ui.separator();
            if let Some(game_config) = game_config {
                for (&client_id, &mass_id) in game_config.client_mass_map.iter() {
                    let color = game_config.init_data.masses.get(&mass_id).unwrap().color;
                    let [r, g, b, a] = color;
                    // FIXME: Because "color" in `InitData` is not done right!
                    // `r,g,b` un-normalized, and yet `a=1.0` ...!
                    let [r, g, b] = Vec3::new(r, g, b).normalize().to_array();
                    let color = Color32::from_rgba_premultiplied(
                        (r * 255.0) as u8,
                        (g * 255.0) as u8,
                        (b * 255.0) as u8,
                        (a * 255.0) as u8,
                    );
                    let nickname = to_nick(client_id).trim_end().to_owned();
                    let prefix = if client_id == my_id { "* " } else { "  " };
                    let line = format!("{prefix}{nickname}");
                    let line = line.to_owned();
                    ui.label(RichText::new(line).color(color).font(FontId {
                        size: 8.0,
                        family: Monospace,
                    }));
                }
            }
        });
}

/// Cleverly position the "objective" camera to get a good view of the masses, using only the masses as input.
///
/// refactor_tags: cameras_write, mass_id_read, gui
pub fn position_objective_camera(
    masses: Query<&Transform, With<components::MassID>>,
    mut cameras: Query<
        (&mut Transform, &Camera, &resources::CameraTag),
        Without<components::MassID>,
    >,
) {
    for (mut transform, _, tag) in cameras.iter_mut() {
        if *tag == resources::CameraTag::Objective {
            // FIXME: what if not is_active?
            let centroid = simulation::get_centroid(
                masses
                    .iter()
                    .map(|t| (scale_to_mass(t.scale), t.translation))
                    .collect::<Vec<_>>(),
            );
            let positions = masses.iter().map(|t| t.translation).collect::<Vec<_>>();
            let mut furthest_two = simulation::FurthestTwo::from(centroid);
            let furthest_two = furthest_two.update(&positions);
            if let Some(triplet_cross) = furthest_two.get_farthest_triplet_normal() {
                let camera_translation_direction = triplet_cross.normalize();
                // FIXME: points.0 is `Some` because `get_farthest_triplet_normal()` is `Some`.
                // A better way forward: Make a type that represents all the masses at a frozen
                // point in time `(mass, location)` and provide methods like `get_centroid()`
                // on downward. We needen't leave people wondering, "What is 'triplet_cross'?"
                let max_centroid_distance = (furthest_two.points.0.unwrap() - centroid).length();
                let camera_translation = camera_translation_direction * max_centroid_distance * 2.0;
                *transform =
                    Transform::from_translation(camera_translation).looking_at(centroid, Vec3::Y);
            } else {
                // FIXME: (yes, please, fix me.) ... this is not working, we'll just put a pin in it!!
                trace!("no triplet!");
            }
        }
    }
}

/// Set the `UiState` resource based upon user input
///
/// refactor_tags: ui_state_write, user_input
pub fn set_ui_state(mut ui_state: ResMut<resources::UiState>, keys: Res<Input<KeyCode>>) {
    if keys.just_released(KeyCode::O) {
        ui_state.camera = match ui_state.camera {
            resources::CameraTag::Objective => resources::CameraTag::Client,
            resources::CameraTag::Client => resources::CameraTag::Objective,
        };
    }
    if keys.just_released(KeyCode::I) {
        ui_state.show_info = !ui_state.show_info;
    }
}

/// Set the active camera based upon `UiState`
///
/// NOTE: Some kind of "assurance" that we have exactly one camera for each
/// of the `CameraTag` values, that only one is active at any time and that
/// the current active camera matches `UiState`.
///
/// refactor_tags: ui_state_read, camera_write, testing
pub fn set_active_camera(
    ui_state: Res<resources::UiState>,
    mut cameras: Query<(&mut Camera, &resources::CameraTag)>,
) {
    // TODO: Who, where, how to assert that we have only one active camera yadda yadda?
    if ui_state.is_changed() || ui_state.is_added() {
        for (mut camera, tag) in cameras.iter_mut() {
            if ui_state.camera == *tag {
                camera.is_active = true;
            } else {
                camera.is_active = false;
            }
        }
    }
}

/// For `client_id`, and `address` supplied by caller, create and return `RenetClient`.
///
/// NOTE: Insertion of the `RenetClient` resource triggers the connection process.
///
/// refactor_tags: network_client_insert
pub fn new_renet_client(client_id: u64, address: String) -> RenetClient {
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

/// Set a helpful window title
///
/// refactor_tags: gui, windows_write, network_client_read
pub fn set_window_title(mut windows: ResMut<Windows>, client: Res<RenetClient>) {
    let title = "Mass Gathering";
    let id = client.client_id();
    let nickname = to_nick(id).trim_end().to_string();
    let title = format!("{title} | nick: \"{nickname}\"");
    windows.primary_mut().set_title(title);
}

/// Set window resolution
///
/// refactor_tags: gui, windows_write, startup_system
pub fn set_resolution(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if cfg!(debug_assertions) {
        debug!("Debug mode, so making your window smaller");
        window.set_resolution(1280.0 / 2.0, 720.0 / 2.0);
    } else {
        window.set_resolution(1280.0, 720.0);
    }
}

/// Spawn one camera for each possible `CameraTag` value, ensure only one is active.
/// Insert the corresponding `CameraTag` marker component into each newly-spawned camera.
///
/// refactor_tags: gui, commands, startup_system, testing
pub fn spawn_cameras(mut commands: Commands) {
    for tag in &[
        resources::CameraTag::Client,
        resources::CameraTag::Objective,
    ] {
        let is_active = *tag == resources::CameraTag::Client;
        let priority = tag.into();
        commands
            .spawn(Camera3dBundle {
                camera: Camera {
                    priority,
                    is_active,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(tag.clone());
    }
}

/// Spawn some reasonable directional lights to light masses.
///
/// refactor_tags: gui, commands, startup_system
pub fn let_light(mut commands: Commands) {
    debug!("Adding some directional lighting (distant suns)");
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(-0.5, -0.3, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 20_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(1.0, -2.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

/// Rotate inhabited mass based upon user input, write `ToServer::Rotation` with the _net_ rotation.
///
/// NOTE: The server broadcasts the current rotation to all _other_ clients, which they use to update their simulation.
///
/// refactor_tags: gui, user_input, to_server_write, inhabited_mass_write, time,
pub fn control(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut to_server_events: EventWriter<events::ToServer>,
    mut inhabitant_query: Query<&mut Transform, With<components::ClientInhabited>>,
) {
    let nudge = TAU / 10000.0;
    let keys_scaling = 10.0;
    let mut rotation = Vec3::ZERO;
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
            let message = events::ToServer::Rotation(transform.rotation);
            to_server_events.send(message);
        }
    }
}

/// For every `ProjectileSpawned` received from the simulation, insert a `PbrBundle` to make it visible.
///
/// refactor_tags: gui, commands, meshes, materials, from_simulation_read
pub fn visualize_projectiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut projectile_spawned_events: EventReader<FromSimulation>,
) {
    for message in projectile_spawned_events.iter() {
        if let &FromSimulation::ProjectileSpawned(entity) = message {
            debug!("Making projectile {entity:?} visible");
            commands
                .entity(entity)
                .insert(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.5,
                        ..Default::default()
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::RED + Color::WHITE * 0.2,
                        emissive: Color::rgb_u8(125, 125, 125),
                        unlit: true,
                        ..Default::default()
                    }),
                    transform: Transform::from_scale(Vec3::ONE * 0.5),
                    ..Default::default()
                })
                .with_children(|children| {
                    children.spawn(PointLightBundle {
                        point_light: PointLight {
                            intensity: 100.0,
                            color: Color::RED,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        }
    }
}

/// Show sights for uninhabited mass, send `ToServer::ProjectileFired` when user fires.
///
/// refactor_tags: gui, uninhabited_mass_read, sights_write, inhabited_mass_read, rapier_context_read, user_input, to_server_write
pub fn handle_projectile_engagement(
    mass_query: Query<
        (&Transform, &components::MassID),
        (
            Without<components::Inhabitable>,
            Without<components::ClientInhabited>,
        ),
    >,
    mut sights_visibility: Query<&mut Visibility, With<components::Sights>>,
    inhabited_mass_query: Query<
        (&Transform, &components::MassID),
        With<components::ClientInhabited>,
    >,
    rapier_context: Res<RapierContext>,
    keys: Res<Input<KeyCode>>,
    mut to_server_events: EventWriter<events::ToServer>,
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
                sights_visibility.for_each_mut(|mut visibility| visibility.is_visible = true);
                if keys.just_pressed(KeyCode::Space) {
                    debug!("User has fired projectile at mass {mass:?}");
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
            }
        } else {
            sights_visibility.for_each_mut(|mut visibility| visibility.is_visible = false);
        }
    } else {
        warn!("ClientInhabited mass not found (yet?)");
    }
}

/// refactor_tags: UNSET
pub fn visualize_masses(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut from_simulation_events: EventReader<FromSimulation>,
    client: Res<RenetClient>,
    game_config: Option<Res<resources::GameConfig>>,
    cameras: Query<(Entity, &resources::CameraTag)>,
) {
    if let Some(game_config) = game_config {
        let client_id = client.client_id();
        for message in from_simulation_events.iter() {
            if let &FromSimulation::MassSpawned {
                entity,
                mass_id,
                mass_init_data,
            } = message
            {
                let mut mass_commands = commands.entity(entity);
                let color: Color = mass_init_data.color.into();
                let transform: Transform = mass_init_data.into();
                mass_commands.insert(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 1.0,
                        ..Default::default()
                    })),
                    material: materials.add(color.into()),
                    transform,
                    ..Default::default()
                });

                let inhabited = mass_id == *game_config.client_mass_map.get(&client_id).unwrap();

                let inhabitable = mass_init_data.inhabitable && !inhabited;

                if inhabited {
                    // rustgods, what's the smart version of:
                    let mut client_camera = None;
                    for (camera, tag) in cameras.iter() {
                        if *tag == resources::CameraTag::Client {
                            client_camera = Some(camera);
                        }
                    }
                    let client_camera = if let Some(c) = client_camera {
                        c
                    } else {
                        panic!("No client camera. Cannot proceed!");
                    };
                    mass_commands.add_child(client_camera);
                }

                mass_commands.with_children(|children| {
                    if inhabited {
                        children
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
                        children
                            .spawn(PointLightBundle {
                                transform: Transform::from_xyz(0.0, 0.0, -0.15),
                                visibility: Visibility::INVISIBLE,
                                point_light: PointLight {
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(components::Sights);
                    }
                    if inhabitable {
                        // barrel
                        children.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Capsule {
                                radius: 0.05,
                                depth: 1.0,
                                ..Default::default()
                            })),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::from_rotation(Quat::from_rotation_x(TAU / 4.0))
                                .with_translation(Vec3::Z * -1.5),
                            ..Default::default()
                        });
                        // horizontal stabilizer
                        children.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Box::new(2.0, 0.075, 1.0))),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::from_translation(Vec3::Z * 0.5),
                            ..Default::default()
                        });
                        // vertical stabilizer
                        children.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Box::new(2.0, 0.075, 1.0))),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::from_rotation(Quat::from_rotation_z(TAU / 4.0))
                                .with_translation(Vec3::Z * 0.5),
                            ..Default::default()
                        });
                    }
                });
            }
        }
    }
}
