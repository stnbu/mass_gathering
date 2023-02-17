/// Physics!!
///
/// Oribity stuff and mass-mass collision stuff (the latter disabled for now).
///
/// Maybe this goes in "simulation"?
use crate::*;
use bevy::ecs::query::WorldQuery;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionEvent, RigidBody, Sensor};

#[derive(Debug)]
pub struct MassCollisionEvent(pub Entity, pub Entity);

/// refactor_tags: UNSET
pub fn handle_mass_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut mass_collision_events: EventWriter<MassCollisionEvent>,
    masses_query: Query<With<components::MassID>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e0, e1, _) = collision_event {
            if masses_query.get_many([*e0, *e1]).is_ok() {
                let event = MassCollisionEvent(*e0, *e1);
                mass_collision_events.send(event);
            }
        }
    }
}

pub struct DespawnMassEvent(pub Entity);

/// refactor_tags: UNSET
pub fn handle_despawn_mass(
    mut commands: Commands,
    mut despawn_mass_events: EventReader<DespawnMassEvent>,
) {
    for &DespawnMassEvent(entity) in despawn_mass_events.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// FIXME: F
/// FIXME: i
/// FIXME: x me! None of this WorldQuery/MergableMass code is tested!
///  At all! The merge code is disabled at the plugin level (missing),
/// and I'm choosing to check in this code that compiles but is
///    UNTESTED
/// as a convenience. Good luck, Future Homer! Angry? Just look at the
/// parent commit. FYI, the below `...Item` stuff is part of WQ macro.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct MergableMass {
    pub entity: Entity,
    pub transform: &'static mut Transform,
    pub momentum: &'static mut components::Momentum,
    pub inhabitation: &'static components::Inhabitation,
}

struct MergeResult<'w> {
    major: MergableMassItem<'w>,
    minor: MergableMassItem<'w>,
}

impl<'w> MergeResult<'w> {
    fn from(pair: [MergableMassItem<'w>; 2]) -> Result<Self, &'static str> {
        let [p0, p1] = pair;
        let (major, minor) = if p0.inhabitation.inhabitable() && p1.inhabitation.inhabitable() {
            return Err("Inhabitable pair collision");
        } else if p0.inhabitation.inhabitable() {
            (p0, p1)
        } else if p1.inhabitation.inhabitable() {
            (p1, p0)
        } else {
            if scale_to_mass(p0.transform.scale) > scale_to_mass(p1.transform.scale) {
                (p0, p1)
            } else {
                (p1, p0)
            }
        };
        Ok(MergeResult { major, minor })
    }

    fn apply_major_delta(&mut self) {
        let combined_momentum = (self.major.momentum.velocity
            * scale_to_mass(self.major.transform.scale))
            + (self.minor.momentum.velocity * scale_to_mass(self.minor.transform.scale));
        let combined_mass =
            scale_to_mass(self.major.transform.scale) + scale_to_mass(self.minor.transform.scale);
        let delta_v = (combined_momentum / combined_mass) - self.major.momentum.velocity;
        let major_factor = scale_to_mass(self.major.transform.scale) / combined_mass;
        let minor_factor = scale_to_mass(self.minor.transform.scale) / combined_mass;
        let weighted_midpoint = ((major_factor * self.major.transform.translation)
            + (minor_factor * self.minor.transform.translation))
            / 2.0;
        let delta_t = weighted_midpoint - self.major.transform.translation;

        let delta_s = if self.major.inhabitation.inhabitable() {
            1.0
        } else {
            major_factor.powf(-1.0 / 3.0)
        };
        self.major.momentum.velocity += delta_v;
        self.major.transform.scale = mass_to_scale(combined_mass);
        self.major.transform.translation += delta_t;
        self.major.transform.scale *= delta_s;
    }

    fn minor_entity(&self) -> Entity {
        self.minor.entity
    }
}

/// refactor_tags: UNSET
pub fn merge_masses(
    mut mergable_masses_query: Query<MergableMass>,
    mut mass_events: EventReader<MassCollisionEvent>,
    mut despawn_mass_events: EventWriter<DespawnMassEvent>,
) {
    for MassCollisionEvent(e0, e1) in mass_events.iter() {
        if let Ok(pair) = mergable_masses_query.get_many_mut([*e0, *e1]) {
            if let Ok(mut merge_result) = MergeResult::from(pair) {
                merge_result.apply_major_delta();
                despawn_mass_events.send(DespawnMassEvent(merge_result.minor_entity()));
            }
        }
    }
}

#[derive(Bundle)]
pub struct PointMassBundle {
    #[bundle]
    pub transform_bundle: TransformBundle,
    pub momentum: components::Momentum,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub active_events: ActiveEvents,
    pub sensor: Sensor,
}

impl Default for PointMassBundle {
    fn default() -> Self {
        Self {
            transform_bundle: Default::default(),
            momentum: Default::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Default::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            sensor: Default::default(),
        }
    }
}

/// refactor_tags: UNSET
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
