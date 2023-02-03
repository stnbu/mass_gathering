use crate::*;
use bevy_rapier3d::prelude::{Collider, CollisionEvent};
use bevy_renet::renet::RenetClient;
use std::time::SystemTime;

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
                if inhabited_mass_id == mass_id {
                    mass_transform.rotation = *rotation;
                    break;
                }
            }
        }
    }
}

pub enum FromSimulation {
    ProjectileSpawned(Entity),
    MassSpawned {
        entity: Entity,
        mass_id: u64,
        mass_init_data: resources::MassInitData,
    },
}

pub fn handle_game_config_insertion(
    mut commands: Commands,
    game_config: Option<Res<resources::GameConfig>>,
    mut from_simulation_events: EventWriter<FromSimulation>,
    client: Res<RenetClient>,
) {
    if let Some(game_config) = game_config {
        if game_config.is_added() {
            /*
            Is it usual/health for one to just let their arms hang when relaxed and upright?

            Or is it "normal" for your arms to be elevated just a bit because of some amount of resting tension in your shoulders?
            */
            let inhabited_mass_id = game_config.client_mass_map.get(&client.client_id());
            for (&mass_id, &mass_init_data) in game_config.init_data.masses.iter() {
                let mass = mass_init_data.mass;
                let position = mass_init_data.motion.position;
                let scale = Vec3::splat(mass_to_radius(mass));
                let mut transform = Transform::from_translation(position).with_scale(scale);
                if mass_init_data.inhabitable {
                    transform.look_at(Vec3::ZERO, Vec3::Y);
                    transform.scale += Vec3::splat(2.5);
                }
                let radius = 1.0;
                let mut mass_commands = commands.spawn(physics::PointMassBundle {
                    transform_bundle: TransformBundle::from_transform(transform),
                    momentum: components::Momentum {
                        velocity: mass_init_data.motion.velocity,
                    },
                    collider: Collider::ball(radius),
                    ..Default::default()
                });
                if inhabited_mass_id.is_some() && mass_id == *inhabited_mass_id.unwrap() {
                    mass_commands.insert(components::ClientInhabited);
                } else {
                    mass_commands.insert(components::Inhabitable);
                }
                mass_commands.insert(components::MassID(mass_id));
                let entity = mass_commands.id();
                from_simulation_events.send(FromSimulation::MassSpawned {
                    entity,
                    mass_id,
                    mass_init_data,
                });
            }
        }
    }
}

pub fn handle_projectile_fired(
    mut commands: Commands,
    mut to_client_events: EventReader<events::ToClient>,
    mut projectile_spawned_events: EventWriter<FromSimulation>,
) {
    for message in to_client_events.iter() {
        if let events::ToClient::ProjectileFired(projectile_flight) = message {
            let radius = 0.5;
            let id = commands
                .spawn(physics::PointMassBundle {
                    transform_bundle: TransformBundle::from_transform(Transform::from_scale(
                        Vec3::ONE * radius,
                    )),
                    ..Default::default()
                })
                .insert(Collider::default())
                .insert(*projectile_flight)
                .id();
            projectile_spawned_events.send(FromSimulation::ProjectileSpawned(id));
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
