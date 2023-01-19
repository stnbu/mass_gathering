// FIXME TODO NOTE WOW: If you start with everything being a radius of 1.0 and then control
// all else by _scaling_ the sphere, you can
// 1. Simplify a lot of code.
// 1. Have the "radius" "stored" in the PBR
// 1. We might be able to get rid of Momentum::mass too and maybe use rapier's Velocity??
use crate::*;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionEvent, RigidBody, Sensor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Resource, Debug, Copy, Clone)]
pub struct PhysicsConfig {
    pub sims_per_frame: u32,
    pub zerog: bool,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            sims_per_frame: 10,
            zerog: false,
        }
    }
}

#[derive(Debug)]
pub struct MassCollisionEvent(pub Entity, pub Entity);

pub fn handle_mass_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut mass_collision_events: EventWriter<MassCollisionEvent>,
    mass_query: Query<With<components::MassID>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e0, e1, _) = collision_event {
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

pub fn merge_masses(
    mut mass_query: Query<(&mut Transform, &mut components::Momentum, Entity)>,
    inhabitant_query: Query<
        Entity,
        Or<(
            With<components::Inhabitable>,
            With<components::ClientInhabited>,
        )>,
    >,
    mut mass_events: EventReader<MassCollisionEvent>,
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

            debug!("Updating major mass {:?}", major.2);
            major.1.velocity += delta_v;
            major.1.mass = combined_mass;
            major.0.translation += delta_p;
            major.0.scale *= delta_s;

            debug!("Signaling despawn request for minor mass {:?}", minor.2);
            despawn_mass_events.send(DespawnMassEvent(minor.2));
        }
    }
}

#[derive(Bundle)]
pub struct PointMassBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub momentum: components::Momentum,
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

pub fn freefall(
    mut masses_query: Query<(Entity, &mut Transform, &mut components::Momentum)>,
    physics_config: Res<PhysicsConfig>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let mut masses = masses_query
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
        masses = masses
            .iter()
            .zip(accelerations)
            .map(|((entity, translation, mass, velocity), force)| {
                (
                    *entity,
                    *translation + *velocity * dt,
                    *mass,
                    *velocity + (force * dt) / *mass,
                )
            })
            .collect::<Vec<_>>();
    }

    for (entity, translation, mass, velocity) in masses.iter() {
        if let Ok((_, mut transform, mut momentum)) = masses_query.get_mut(*entity) {
            transform.translation = *translation;
            momentum.velocity = *velocity;
            momentum.mass = *mass;
        }
    }
}
