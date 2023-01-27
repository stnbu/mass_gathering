use bevy_rapier3d::prelude::{Collider, CollisionEvent, QueryFilter, RapierContext};
use bevy_renet::{
    renet::{ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig},
    run_if_client_connected, RenetClientPlugin,
};
use clap::Parser;
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
    mut game_state: ResMut<State<resources::GameState>>,
    mut to_client_events: EventReader<events::ToClient>,
    client: Res<RenetClient>,
) {
    let my_id = client.client_id();
    for message in to_client_events.iter() {
        match message {
            events::ToClient::SetGameState(new_game_state) => {
                let _ = game_state.overwrite_set(*new_game_state);
            }
            events::ToClient::InhabitantRotation { .. } => {
                // handled by separate system
            }
            events::ToClient::SetGameConfig(game_config) => {
                let inhabited_mass_id = *game_config.client_mass_map.get(&my_id).unwrap();
                game_config
                    .init_data
                    .spawn_masses(&mut commands, Some(inhabited_mass_id));
                commands.insert_resource(game_config.clone());
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

//
// Below here is PBR/visual stuff that is being [re]introduced
//

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
        window.set_resolution(1280.0 / 2.0, 720.0 / 2.0);
    } else {
        window.set_resolution(1280.0, 720.0);
    }
}

pub fn visualize_masses(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_config: Res<resources::GameConfig>,
    masses_query: Query<(
        Entity,
        &components::MassID,
        Option<&components::Inhabitable>,
        Option<&components::ClientInhabited>,
    )>,
    mut has_run: Local<bool>,
) {
    // FIXME: [HACK] Relying on `bool` having a default of `false`. The goal being "run once"
    if !*has_run && !masses_query.is_empty() {
        *has_run = true;
        for (&mass_id, &resources::MassInitData { color, .. }) in
            game_config.init_data.masses.iter()
        {
            for (entity, &components::MassID(this_mass_id), inhabitable, inhabited) in
                masses_query.iter()
            {
                let inhabitable = inhabitable.is_some();
                let inhabited = inhabited.is_some();
                assert!(!(inhabitable && inhabited));
                let color: Color = color.into();
                if this_mass_id == mass_id {
                    warn!("Visualizing {mass_id}");
                    commands
                        .entity(entity)
                        .insert(VisibilityBundle::default())
                        // .insert(Visibility::default())
                        // .insert(ComputedVisibility::INVISIBLE)
                        .with_children(|children| {
                            // mass surface
                            children.spawn(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Icosphere {
                                    radius: 1.0,
                                    ..Default::default()
                                })),
                                material: materials.add(color.into()),
                                ..Default::default()
                            });
                            if inhabited {
                                warn!("Mass {mass_id} is inhabted");
                                children.spawn(Camera3dBundle::default());
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
                                warn!("Mass {mass_id} is inhabtable");
                                // barrel
                                children.spawn(PbrBundle {
                                    mesh: meshes.add(Mesh::from(shape::Capsule {
                                        radius: 0.05,
                                        depth: 1.0,
                                        ..Default::default()
                                    })),
                                    material: materials.add(Color::WHITE.into()),
                                    transform: Transform::from_rotation(Quat::from_rotation_x(
                                        TAU / 4.0,
                                    ))
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
                                    transform: Transform::from_rotation(Quat::from_rotation_z(
                                        TAU / 4.0,
                                    ))
                                    .with_translation(Vec3::Z * 0.5),
                                    ..Default::default()
                                });
                            }
                        });
                    // We found/are done looking for the mass_id in question.
                    break;
                }
            }
        }
    }
}

//
// Below here to go to "simulation.rs"
//

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
                debug!("Mass {to_mass_id} is now in our sights");
                // FIXME: If the "fire" button has been pressed...
                // this whole func may be out of the scope of "simulation".
                // as its all about turning user input into messages to the
                // server!
                if false {
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
        }
    } else {
    }
}

pub fn handle_projectile_fired(
    mut to_client_events: EventReader<events::ToClient>,
    mut commands: Commands,
) {
    for message in to_client_events.iter() {
        if let events::ToClient::ProjectileFired(projectile_flight) = message {
            let radius = 0.5;
            commands
                .spawn(physics::PointMassBundle {
                    transform_bundle: TransformBundle::from_transform(Transform::from_scale(
                        Vec3::ONE * radius,
                    )),
                    ..Default::default()
                })
                .insert(Collider::default())
                .insert(*projectile_flight);
        }
    }
}

pub fn move_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Transform, &events::ProjectileFlight)>,
    masses_query: Query<(&Transform, &components::MassID), Without<events::ProjectileFlight>>,
) {
    let proportion_of = 1.0 / 512.0;
    let portions_per_second = 128.0 * 3.0;

    for (projectile_id, mut projectile_transform, projectile_flight) in projectile_query.iter_mut()
    {
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

pub fn handle_projectile_collision(
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
                    debug!("Projectile collided: {projectile_flight:?}");
                }
            }
        }
    }
}

// // // // // // // // // // //

// -- inhabit[ed|able] mass PBR parts ...

/*
    // mass surface
    children.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius,
            ..Default::default()
        })),
        material: materials.add(color.into()),
        ..Default::default()
    });
    // sights
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
    // sights
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
*/

// -- other ...

/*
// // PROJECTILE // //
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.5,
                        ..Default::default()
                    })),
                    visibility: Visibility::INVISIBLE,
                    material: materials.add(StandardMaterial {
                        base_color: Color::RED + Color::WHITE * 0.2,
                        emissive: Color::rgb_u8(125, 125, 125),
                        unlit: true,
                        ..default()
                    }),
                    transform: Transform::from_scale(Vec3::ONE * 0.5),
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
*/

/*
// // EXPLOSION // //
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
*/
