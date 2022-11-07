use crate::craft::BallisticProjectileTarget;
use crate::DespawnTimer;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionEvent, RigidBody, Sensor};
use std::{f32::consts::PI, time::Duration};

pub struct PhysicsConfig {
    pub trails: bool,
    pub sims_per_frame: u8,
    pub trail_ttl: u64,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            trails: false,
            sims_per_frame: 10,
            trail_ttl: 2500,
        }
    }
}

pub fn handle_planet_collisions(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut planet_query: Query<(&mut Transform, &mut Momentum, Entity, &mut Collider)>,
    mut target_query: Query<(&mut BallisticProjectileTarget, Entity)>,
) {
    // FIXME -- need to handle 3-way collisions IF it ever happens...
    // FIXME -- "projectiles" still needlessly show up in planet_collider_query
    //          because we are not filtering `events` enough/at all.
    for collision_event in events.iter() {
        if let CollisionEvent::Started(e0, e1, _) = collision_event {
            if let Ok([p0, p1]) = planet_query.get_many_mut([*e0, *e1]) {
                debug!(
                    "Collision-started event for planets {:?} and {:?}",
                    p0.2, p1.2
                );
                let (mut major, minor, cull) = if p0.1.mass > p1.1.mass {
                    (p0, p1, e1)
                } else if p0.1.mass < p1.1.mass {
                    (p1, p0, e0)
                } else {
                    // FIXME -- a fair tie-breaker
                    warn!("Colliding planets {:?} and {:?} have exactly the same mass. Picking major/minor arbitrarilly.", e0, e1);
                    (p0, p1, e1)
                };

                // Merge Math
                let major_factor = major.1.mass / (major.1.mass + minor.1.mass);
                let minor_factor = minor.1.mass / (major.1.mass + minor.1.mass);
                let scale_up = (mass_to_radius(major.1.mass) + mass_to_radius(minor.1.mass))
                    / mass_to_radius(major.1.mass);
                major.1.mass += minor.1.mass;
                major.1.velocity =
                    major.1.velocity * major_factor + minor.1.velocity * minor_factor;
                major.0.translation =
                    (major_factor * major.0.translation) + (minor_factor * minor.0.translation);
                major.0.scale = scale_up * Vec3::splat(1.0);
                // End Merge Math

                for (mut target, projectile_id) in target_query.iter_mut() {
                    if target.planet == *cull {
                        warn!("Projectile {projectile_id:?} has despawned planet {:?} as its target. Remapping to merge-ee planet {:?}", target.planet, major.2);
                        target.planet = major.2;
                    }
                }
                debug!("despawning planet {:?}", cull);
                commands.entity(*cull).despawn_recursive();
            } else {
                debug!("One of {:?} or {:?} has no parent.", e0, e1);
            }
        }
    }
}

fn radius_to_mass(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * radius.powf(3.0)
}

fn mass_to_radius(mass: f32) -> f32 {
    ((mass * (3.0 / 4.0)) / PI).powf(1.0 / 3.0)
}

#[derive(Bundle)]
pub struct PlanetBundle {
    #[bundle]
    pbr: PbrBundle,
    momentum: Momentum,
    rigid_body: RigidBody,
    collider: Collider,
    active_events: ActiveEvents,
    sensor: Sensor,
}

impl Default for PlanetBundle {
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

pub fn spawn_planet<'a>(
    radius: f32,
    position: Vec3,
    velocity: Vec3,
    color: Color,
    commands: &'a mut Commands,
    meshes: &'a mut ResMut<Assets<Mesh>>,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
) {
    let mass = radius_to_mass(radius);
    let planet_bundle = PlanetBundle {
        pbr: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius,
                ..default()
            })),
            material: materials.add(color.into()),
            transform: Transform::from_translation(position),
            ..default()
        },
        momentum: Momentum { velocity, mass },
        collider: Collider::ball(radius),
        ..Default::default()
    };
    let planet_id = commands.spawn_bundle(planet_bundle).id();
    debug!("Spawned planet={planet_id:?}");
}

#[derive(Component, Debug)]
pub struct Momentum {
    pub velocity: Vec3,
    pub mass: f32,
}

impl Default for Momentum {
    fn default() -> Self {
        Momentum {
            velocity: Vec3::ZERO,
            mass: 0.0,
        }
    }
}

pub fn freefall(
    mut planet_query: Query<(Entity, &mut Transform, &mut Momentum)>,
    time: Res<Time>,
    physics_config: Res<PhysicsConfig>,
) {
    let dt = time.delta_seconds();
    let mut masses = planet_query
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
        if let Ok((id, mut transform, mut momentum)) = planet_query.get_mut(*entity) {
            debug!("Updating translation and momentum for planet {id:?}");
            transform.translation = *translation;
            momentum.velocity = *velocity;
            momentum.mass = *mass;
        }
    }
}

use std::collections::HashMap;
#[derive(Default)]
pub struct PreviousLocations(pub HashMap<Entity, Vec3>);

pub struct BreadcrumbEvent {
    entity: Entity,
    location: Vec3,
}

#[derive(Component)]
pub struct Breadcrumb(pub Entity);

pub fn signal_breadcrumbs(
    planet_query: Query<(Entity, &Transform), With<Momentum>>,
    mut previous_locations: Local<PreviousLocations>,
    mut breadcrumb_events: EventWriter<BreadcrumbEvent>,
) {
    for (entity, transform) in planet_query.iter() {
        if let Some(prev) = previous_locations.0.get(&entity) {
            if (transform.translation - *prev).length() > 2.0 / 3.0 {
                breadcrumb_events.send(BreadcrumbEvent {
                    entity,
                    location: transform.translation,
                });
                previous_locations.0.insert(entity, transform.translation);
            }
        } else {
            previous_locations.0.insert(entity, transform.translation);
        }
    }
}

pub fn spawn_breadcrumbs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut breadcrumb_events: EventReader<BreadcrumbEvent>,
) {
    for event in breadcrumb_events.iter() {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.1,
                    ..Default::default()
                })),
                transform: Transform::from_translation(event.location),
                visibility: Visibility { is_visible: false },
                material: materials.add((Color::WHITE).into()),
                ..Default::default()
            })
            .insert(DespawnTimer {
                ttl: Timer::new(Duration::from_millis(3000), false),
            })
            .insert(Breadcrumb(event.entity));
    }
}
