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

pub fn handle_set_game_config(
    mut commands: Commands,
    mut to_client_events: EventReader<events::ToClient>,
) {
    for message in to_client_events.iter() {
        if let events::ToClient::SetGameConfig(game_config) = message {
            debug!("GameConfig received. Inserting as resource: {game_config:#?}");
            commands.insert_resource(game_config.clone());
        }
    }
}

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

pub fn receive_messages_from_server(
    mut client: ResMut<RenetClient>,
    mut to_client_events: EventWriter<events::ToClient>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
        to_client_events.send(bincode::deserialize(&message).unwrap());
    }
}

use bevy::prelude::{TextBundle, TextStyle};

#[derive(Component)]
pub struct InfoText;

pub fn spawn_info_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let lines = &["line one", "line two"];
    commands
        .spawn(
            TextBundle::from_sections(lines.iter().map(|line| {
                let value = line.to_string();
                let style = TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                };
                TextSection { value, style }
            }))
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .insert(InfoText);
}

pub fn toggle_info_text(
    mut info_text_query: Query<&mut Visibility, With<InfoText>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_released(KeyCode::I) {
        debug!("Toggling info text visibility");
        for mut visibility in info_text_query.iter_mut() {
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

pub fn position_objective_camera(
    masses: Query<&Transform, With<components::MassID>>,
    mut objective_camera: Query<
        (&mut Transform, &Camera),
        (With<ObjectiveCamera>, Without<components::MassID>),
    >,
) {
    if let Ok((mut transform, camera)) = objective_camera.get_single_mut() {
        if camera.is_active {
            let centroid = simulation::get_centroid(
                masses
                    .iter()
                    .map(|t| (scale_to_mass(t.scale), t.translation))
                    .collect::<Vec<_>>(),
            );
            let positions = masses.iter().map(|t| t.translation).collect::<Vec<_>>();
            let mut furthest_two = simulation::FurthestTwo::from(centroid);
            let furthest_two = furthest_two.update(&positions);
            let camera_translation = centroid
                + furthest_two
                    .get_farthest_triplet_normal()
                    .unwrap()
                    .normalize()
                    * (furthest_two.points.0.unwrap() - centroid).length()
                    * 2.0;
            let camera_transform =
                Transform::from_translation(camera_translation).looking_at(centroid, Vec3::Y);
            *transform = camera_transform;
        }
    }
}

pub fn choose_camera(
    mut objective_camera: Query<&mut Camera, (With<ObjectiveCamera>, Without<ClientCamera>)>,
    mut client_camera: Query<&mut Camera, (With<ClientCamera>, Without<ObjectiveCamera>)>,
    keys: Res<Input<KeyCode>>,
) {
    if let (Ok(mut objective_camera), Ok(mut client_camera)) = (
        objective_camera.get_single_mut(),
        client_camera.get_single_mut(),
    ) {
        assert!(
            objective_camera.is_active ^ client_camera.is_active,
            "Expected exactly one camera to be active!"
        );
        if keys.just_released(KeyCode::O) {
            debug!("Swapping client/objective cameras");
            objective_camera.is_active = !objective_camera.is_active;
            client_camera.is_active = !client_camera.is_active;
        }
    }
}

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

pub fn set_window_title(mut windows: ResMut<Windows>, client: Res<RenetClient>) {
    let title = "Mass Gathering";
    let id = client.client_id();
    let nickname = to_nick(id).trim_end().to_string();
    let title = format!("{title} | nick: \"{nickname}\"");
    windows.primary_mut().set_title(title);
}

pub fn set_resolution(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if cfg!(debug_assertions) {
        debug!("Debug mode, so making your window smaller");
        window.set_resolution(1280.0 / 2.0, 720.0 / 2.0);
    } else {
        window.set_resolution(1280.0, 720.0);
    }
}

pub fn spawn_objective_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                priority: OBJECTIVE_CAMERA_PRIORITY,
                is_active: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ObjectiveCamera);
}

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

#[derive(Component)]
pub struct ClientCamera;
const CLIENT_CAMERA_PRIORITY: isize = 0;

#[derive(Component)]
pub struct ObjectiveCamera;
const OBJECTIVE_CAMERA_PRIORITY: isize = 1;

pub fn visualize_masses(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut from_simulation_events: EventReader<FromSimulation>,
    client: Res<RenetClient>,
    game_config: Option<Res<resources::GameConfig>>,
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
                debug!("Making mass {mass_id} ({entity:?}) visible");
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

                if !mass_init_data.inhabitable {
                    debug!("Mass {mass_id} is uninhabitable");
                }

                let inhabited = mass_id == *game_config.client_mass_map.get(&client_id).unwrap();

                let inhabitable = mass_init_data.inhabitable && !inhabited;

                mass_commands.with_children(|children| {
                    if inhabited {
                        let nickname = to_nick(client_id).trim_end().to_string();
                        debug!("Mass {mass_id} is inhabited by us, {nickname}");
                        children
                            .spawn(Camera3dBundle {
                                camera: Camera {
                                    priority: CLIENT_CAMERA_PRIORITY,
                                    //is_active: false,
                                    is_active: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(ClientCamera);
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
                        debug!("Mass {mass_id} is inhabitable");
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
