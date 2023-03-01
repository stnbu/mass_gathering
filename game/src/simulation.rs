/// Everything minus the visuals. Move stuff, spawn stuff, but nothing that's not headless.
use crate::*;
use bevy_rapier3d::prelude::{Collider, CollisionEvent};

use std::time::SystemTime;

/// refactor_tags: simulation, collision_event_read, projectile_read, masses_read, disabled
pub fn rotate_inhabitable_masses(
    player: Res<components::Player>,
    mut to_client_events: EventReader<events::ToClient>,
    mut masses_query: Query<(&mut Transform, &components::Inhabitable)>,
) {
    for message in to_client_events.iter() {
        if let events::ToClient::InhabitantRotation { rotation, .. } = message {
            // for ... masses inhabited by other players
            for (mut mass_transform, inhabitation) in masses_query.iter_mut() {
                if !inhabitation.by(*player) {
                    mass_transform.rotation = *rotation;
                }
            }
        }
    }
}

/// refactor_tags: simulation, from_simulation, user_input
pub enum FromSimulation {
    ProjectileSpawned(Entity),
    MassSpawned {
        entity: Entity,
        mass_id: u64,
        mass_init_data: resources::MassInitData,
    },
}

/// React to the insertion of the GameConfig resource.
///   * Iterate through the masses and spawn `TransformBundle`s that are
///   the basis of the simulation. These are "invisible" such that we can
///   skip all GUI stuff and use this in a "headless" sense.
///   * Conditionally handle "inhabited" and complicated stuff.
///   * Emit an event for each new entity. Other systems can respond by.
///   Upgrading them to "visible" so they can be rendered and displayed.
///   * We do not care about seeing them but we do care how "big" they are
///   because we insert a `Collider`.
///
/// refactor_tags: simulation, running, eventwriter, client
pub fn handle_game_config_insertion(
    mut commands: Commands,
    game_config: Option<Res<resources::GameConfig>>,
    mut from_simulation_events: EventWriter<FromSimulation>,
) {
    if let Some(game_config) = game_config {
        if game_config.is_added() {
            debug!("GameConfig resource is_added");
            for (&mass_id, &mass_init_data) in game_config.init_data.masses.iter() {
                let transform: Transform = mass_init_data.into();
                let radius = 1.0;
                let inhabitation = mass_init_data.inhabitation;
                debug!("Spawining PointMassBundle for mass {mass_id}");
                // FIXME: If we could de/serialize "all the parts of the game", we could:
                //   1) Just send that instead of "GameConfig" and all that.
                //   2) We could serialize the whole "PointMassBundle" below and send that
                //      in a message "EntitySpawned" (or something) for handling visuals.
                let entity = commands
                    .spawn(physics::PointMassBundle {
                        transform_bundle: TransformBundle::from_transform(transform),
                        momentum: components::Momentum {
                            velocity: mass_init_data.motion.velocity,
                        },
                        collider: Collider::ball(radius),
                        ..Default::default()
                    })
                    .insert(inhabitation)
                    .insert(components::MassID(mass_id))
                    .id();
                from_simulation_events.send(FromSimulation::MassSpawned {
                    entity,
                    mass_id,
                    mass_init_data,
                });
            }
        }
    }
}

/// refactor_tags: simulation, commands, to_client_read, from_simulation_write
pub fn handle_projectile_fired(
    mut commands: Commands,
    mut to_client_events: EventReader<events::ToClient>,
    mut projectile_spawned_events: EventWriter<FromSimulation>,
) {
    for message in to_client_events.iter() {
        if let events::ToClient::ProjectileFired(projectile_flight) = message {
            let radius = 0.5;
            debug!("Spawning TransformBundle for ProjectileFired event");
            let id = commands
                .spawn(TransformBundle::from_transform(Transform::from_scale(
                    Vec3::ONE * radius,
                )))
                .insert(Collider::default())
                .insert(*projectile_flight)
                .id();
            projectile_spawned_events.send(FromSimulation::ProjectileSpawned(id));
        }
    }
}

/// NOTE: Need to give some thought to the use of system time vs bevy's ideas about time. _think about time_
///
/// refactor_tags: simulation, commands, to_client_read, from_simulation_write, projectiles_write, system_time
pub fn move_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Transform, &events::ProjectileFlight)>,
    // this could/should exclude inhabit[ed|able]
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

/// refactor_tags: simulation, collision_event_read, projectile_read, masses_read, disabled
pub fn handle_projectile_collision(
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<&events::ProjectileFlight>,
    masses_query: Query<(With<components::MassID>, Without<components::Inhabitable>)>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e0, e1, _) = collision_event {
            let e0_is_projectile = projectile_query.contains(*e0);
            let e1_is_projectile = projectile_query.contains(*e1);
            if e0_is_projectile ^ e1_is_projectile {
                let projectile_id = if e0_is_projectile { e0 } else { e1 };
                let projectile_flight = projectile_query.get(*projectile_id).unwrap();
                let mass_id = if !e0_is_projectile { e0 } else { e1 };
                if masses_query.contains(*mass_id) {
                    debug!("Projectile collided: {projectile_flight:?}");
                }
            }
        }
    }
}

/// refactor_tags: refactor, simulation, cameras
pub fn get_centroid(masses: Vec<(f32, Vec3)>) -> Vec3 {
    let total_mass = masses
        .iter()
        .fold(0.0, |accumulator, mass| accumulator + mass.0);
    masses.iter().fold(Vec3::ZERO, |accumulator, mass| {
        accumulator + mass.1 * mass.0
    }) / total_mass
}

#[derive(Debug)]
/// refactor_tags: refactor, simulation, cameras
pub struct FurthestTwo {
    pub points: (Option<Vec3>, Option<Vec3>),
    pub reference: Vec3,
}

impl FurthestTwo {
    pub fn from(reference: Vec3) -> Self {
        FurthestTwo {
            points: (None, None),
            reference,
        }
    }

    // FIXME: Not quite right. `>=` doesn't cover discontinuities like origin vs -1 vs +1
    // and more.
    pub fn update(&mut self, positions: &[Vec3]) -> &mut Self {
        for &position in positions.iter() {
            if let Some(challenger) = self.points.0 {
                if (self.reference - position).length() >= challenger.length() {
                    self.points.1 = Some(challenger);
                    self.points.0 = Some(position);
                } else {
                    if let Some(challenger) = self.points.1 {
                        if (self.reference - position).length() >= challenger.length() {
                            self.points.1 = Some(position);
                        }
                    }
                }
            } else {
                self.points.0 = Some(position);
            }
        }
        self
    }

    pub fn get_farthest_triplet_normal(&self) -> Option<Vec3> {
        if let Some(p0) = self.points.0 {
            if let Some(p1) = self.points.1 {
                return Some((p0 - self.reference).cross(p1 - self.reference));
            }
        }
        None
    }
}
