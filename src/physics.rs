use bevy::prelude::*;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionEvent, RigidBody, Sensor};

#[derive(Resource)]
pub struct PhysicsConfig {
    pub sims_per_frame: u32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self { sims_per_frame: 10 }
    }
}

#[derive(Debug)]
pub struct MassCollisionEvent(pub Entity, pub Entity);

pub fn handle_mass_collisions(
    mut events: EventReader<CollisionEvent>,
    mut mass_collision_events: EventWriter<MassCollisionEvent>,
    mass_query: Query<(&Transform, &Momentum)>,
) {
    for collision_event in events.iter() {
        // FIXME: Filter events (for "Sensor")
        if let CollisionEvent::Started(e0, e1, flags) = collision_event {
            debug!(
                "CollisionEvent::Started({:?}, {:?}, flags={:?})",
                e0, e1, flags
            );
            if mass_query.get_many([*e0, *e1]).is_ok() {
                let event = MassCollisionEvent(*e0, *e1);
                debug!("Sending mass collision event: {event:?}");
                mass_collision_events.send(event);
            }
        }
    }
}

pub struct DespawnMassEvent(pub Entity);

pub fn handle_despawn_mass(
    mut commands: Commands,
    mut despawn_mass_events: EventReader<DespawnMassEvent>,
) {
    for &DespawnMassEvent(entity) in despawn_mass_events.iter() {
        debug!("RECURSIVELY despawning mass {entity:?}");
        commands.entity(entity).despawn_recursive();
    }
}

use crate::networking::client::Inhabitable;

pub fn merge_masses(
    mut mass_query: Query<(&Transform, &mut Momentum, Entity)>,
    inhabitant_query: Query<Entity, With<Inhabitable>>,
    mut mass_events: EventReader<MassCollisionEvent>,
    mut delta_events: EventWriter<DeltaEvent>,
    mut despawn_mass_events: EventWriter<DespawnMassEvent>,
) {
    for MassCollisionEvent(e0, e1) in mass_events.iter() {
        let involved_inhabitant = inhabitant_query.iter_many([*e0, *e1]).collect::<Vec<_>>();
        if involved_inhabitant.len() == 2 {
            debug!("Inhabited masses {e0:?} and {e1:?} collided. Ignoring.");
            continue;
        }
        let inhabitant = if involved_inhabitant.len() == 1 {
            let id = involved_inhabitant[0];
            debug!("Inhabited mass {id:?} was involved in a collision.");
            Some(id)
        } else {
            None
        };

        // FIXME: We have write access to `Momentum` and yet we update
        // `delta_v` via an event. Just update it here? Should `DeltaEvent`
        // even have a `delta_v` field?
        if let Ok([p0, p1]) = mass_query.get_many_mut([*e0, *e1]) {
            let (mut major, mut minor) = if p0.1.mass > p1.1.mass {
                (p0, p1)
            // FIXME: tie-breaker!
            } else {
                (p1, p0)
            };

            debug!(
                "We have major-mass={:?} and minor-mass={:?}",
                major.1.mass, minor.1.mass
            );

            if let Some(c) = inhabitant {
                if c == minor.2 {
                    debug!("Inhabited mass {c:?} was the 'minor'! Swapping major for minor.");
                    (major, minor) = (minor, major);
                } else {
                    debug!(
                        "Inhabited mass {c:?} was the 'major', which should be '{:?}'. No swapping.",
                        major.2
                    )
                }
            }

            debug!("Collision of masses:");
            debug!(" Major mass {:?}", major.2);
            debug!("  position: {:?}", major.0.translation);
            debug!("  velocity: {:?}", major.1.velocity);
            debug!("  mass: {:?}", major.1.mass);
            debug!(" Minor mass {:?}", minor.2);
            debug!("  position: {:?}", minor.0.translation);
            debug!("  velocity: {:?}", minor.1.velocity);
            debug!("  mass: {:?}", minor.1.mass);

            let combined_momentum =
                (major.1.velocity * major.1.mass) + (minor.1.velocity * minor.1.mass);
            let combined_mass = major.1.mass + minor.1.mass;
            let delta_v = (combined_momentum / combined_mass) - major.1.velocity;
            // Convince yourself that the sum of these must equal 1.0;
            let major_factor = major.1.mass / combined_mass;
            let minor_factor = minor.1.mass / combined_mass;
            debug!(
                "Directly setting mass of major mass {:?} to {combined_mass:?}",
                major.2
            );
            // Maybe increment mass via an event to?
            major.1.mass = combined_mass;
            let entity = major.2;

            let weighted_midpoint =
                ((major_factor * major.0.translation) + (minor_factor * minor.0.translation)) / 2.0;
            debug!(
                "The weighted midpoint between masses major={:?} and minor={:?} is {weighted_midpoint:?}",
                major.2, minor.2
            );
            let delta_p = weighted_midpoint - major.0.translation;
            // How much to scale in the linear (multiply major original
            // radius by this much to achieve a proportionate mass (i.e. volume) increase.
            // We do _not_ scale inhabited masses (they just get denser nom nom.)
            let delta_s = if inhabitant.is_some() {
                1.0
            } else {
                major_factor.powf(-1.0 / 3.0)
            };
            let event = DeltaEvent {
                entity,
                delta_p,
                delta_v,
                delta_s,
                force_ro: Vec3::ZERO,
            };
            debug!("Sending event: {event:?}");
            delta_events.send(event);
            debug!("Signaling despawn request for minor mass {:?}", minor.2);
            despawn_mass_events.send(DespawnMassEvent(minor.2));
        }
    }
}

#[derive(Bundle)]
pub struct PointMassBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub momentum: Momentum,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub active_events: ActiveEvents,
    pub sensor: Sensor,
}

impl Default for PointMassBundle {
    fn default() -> Self {
        Self {
            pbr: Default::default(),
            momentum: Default::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Default::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            sensor: Default::default(),
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Momentum {
    pub velocity: Vec3,
    pub mass: f32,
    pub force_ro: Vec3,
}

#[derive(Debug)]
pub struct DeltaEvent {
    pub entity: Entity,
    pub delta_p: Vec3,
    pub delta_v: Vec3,
    pub delta_s: f32,
    pub force_ro: Vec3,
}

pub fn signal_freefall_delta(
    mass_query: Query<(Entity, &Transform, &Momentum)>,
    time: Res<Time>,
    physics_config: Res<PhysicsConfig>,
    mut delta_events: EventWriter<DeltaEvent>,
) {
    let dt = time.delta_seconds();
    let mut masses = mass_query
        .iter()
        .map(|t| (t.0, t.1.translation, t.2.mass, t.2.velocity))
        .collect::<Vec<_>>();
    for _ in (0..physics_config.sims_per_frame).rev() {
        let accelerations = masses.iter().map(|particle1| {
            masses.iter().fold(Vec3::ZERO, |acceleration, particle2| {
                let dir = particle2.1 - particle1.1;
                let mag_2 = dir.length();
                let grav_acc = if mag_2 != 0.0 {
                    dir * particle2.2 / (mag_2 * mag_2.sqrt())
                } else {
                    dir
                };
                acceleration + grav_acc * 0.001
            })
        });
        // What would happen if each pass re-re-randomized the order of masses? Good?
        // Actually, ordering by mass might be a thing too.
        masses = masses
            .iter()
            .zip(accelerations)
            .map(|((entity, translation, mass, velocity), acceleration)| {
                let force = acceleration * dt;
                let delta_p = *velocity * dt;
                let delta_v = if *mass == 0.0 {
                    Vec3::ZERO
                } else {
                    force / *mass
                };
                let delta_s = 1.0;
                delta_events.send(DeltaEvent {
                    entity: *entity,
                    delta_p,
                    delta_v,
                    delta_s,
                    force_ro: force,
                });
                (*entity, *translation + delta_p, *mass, *velocity + delta_v)
            })
            .collect::<Vec<_>>();
    }
}

pub fn handle_freefall(
    mut mass_query: Query<(&mut Transform, &mut Momentum)>,
    mut delta_events: EventReader<DeltaEvent>,
) {
    for event in delta_events.iter() {
        if let Ok((mut transform, mut momentum)) = mass_query.get_mut(event.entity) {
            transform.translation += event.delta_p;
            momentum.velocity += event.delta_v;
            momentum.force_ro = event.force_ro * momentum.mass;
            transform.scale *= event.delta_s;
        }
    }
}
