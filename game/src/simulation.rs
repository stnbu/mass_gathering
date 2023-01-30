use crate::*;
use bevy_rapier3d::prelude::{Collider, CollisionEvent, QueryFilter, RapierContext};
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
    if inhabited_mass_query.is_empty() {
        error!("No `ClientInhabited` masses found!");
    }
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
            }
        }
    }
}

pub enum FromSimulation {
    ProjectileSpawned(Entity),
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
