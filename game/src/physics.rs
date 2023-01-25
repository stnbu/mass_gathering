use crate::*;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionEvent, RigidBody, Sensor};

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
        commands.entity(entity).despawn_recursive();
    }
}

// FIXME:
// * Imlement type that can replace the `(Transform, Momentum)` below, so we
//   don't use stupid names like `p0`.
// * It should be possible to compare them `T1 > T2` (compare mass and obey
//   rules about inhabited masses.)
// * Define and implement an "extension type" so you can `set_mass()` and
//   `get_mass` on type `Transform` (using `scale`).

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
            continue;
        }
        let inhabitant = if involved_inhabitant.len() == 1 {
            let id = involved_inhabitant[0];
            Some(id)
        } else {
            None
        };

        if let Ok([p0, p1]) = mass_query.get_many_mut([*e0, *e1]) {
            // scale_to_mass(.0.scale)
            let (mut major, mut minor) = if scale_to_mass(p0.0.scale) > scale_to_mass(p1.0.scale) {
                (p0, p1)
            // FIXME: tie-breaker!
            } else {
                (p1, p0)
            };

            if let Some(c) = inhabitant {
                if c == minor.2 {
                    (major, minor) = (minor, major);
                }
            }

            let combined_momentum = (major.1.velocity * scale_to_mass(major.0.scale))
                + (minor.1.velocity * scale_to_mass(minor.0.scale));
            let combined_mass = scale_to_mass(major.0.scale) + scale_to_mass(minor.0.scale);
            let delta_v = (combined_momentum / combined_mass) - major.1.velocity;
            // Convince yourself that the sum of these must equal 1.0;
            let major_factor = scale_to_mass(major.0.scale) / combined_mass;
            let minor_factor = scale_to_mass(minor.0.scale) / combined_mass;

            let weighted_midpoint =
                ((major_factor * major.0.translation) + (minor_factor * minor.0.translation)) / 2.0;
            let delta_p = weighted_midpoint - major.0.translation;
            // How much to scale in the linear (multiply major original
            // radius by this much to achieve a proportionate mass (i.e. volume) increase.
            // We do _not_ scale inhabited masses (they just get denser nom nom.)
            let delta_s = if inhabitant.is_some() {
                1.0
            } else {
                major_factor.powf(-1.0 / 3.0)
            };
            major.1.velocity += delta_v;
            major.0.scale = mass_to_scale(combined_mass);
            major.0.translation += delta_p;
            major.0.scale *= delta_s;

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
    game_config: Res<resources::GameConfig>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let mut masses = masses_query
        .iter()
        .map(|t| (t.0, t.1.translation, scale_to_mass(t.1.scale), t.2.velocity))
        .collect::<Vec<_>>();
    for _ in (0..game_config.physics_config.speed).rev() {
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
            transform.scale = mass_to_scale(*mass);
        }
    }
}
