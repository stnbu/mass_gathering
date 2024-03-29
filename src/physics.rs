use crate::craft::{ProjectileCollisionEvent, ProjectileTarget};
use crate::{mass_to_radius, radius_to_mass};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionEvent, RigidBody, Sensor};

#[derive(Resource)]
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

#[derive(Debug, Event)]
pub struct PlanetCollisionEvent(pub Entity, pub Entity);

pub fn handle_planet_collisions(
    mut events: EventReader<CollisionEvent>,
    mut projectile_collision_events: EventWriter<ProjectileCollisionEvent>,
    mut planet_collision_events: EventWriter<PlanetCollisionEvent>,
    planet_query: Query<(&Transform, &Momentum)>,
    projectile_query: Query<&Transform, With<ProjectileTarget>>,
) {
    for collision_event in events.read() {
        // FIXME: Filter events (for "Sensor")
        if let CollisionEvent::Started(e0, e1, _) = collision_event {
            if planet_query.get_many([*e0, *e1]).is_ok() {
                let event = PlanetCollisionEvent(*e0, *e1);
                debug!("Sending planet collision event: {event:?}");
                planet_collision_events.send(event);
            } else {
                for (&projectile, &planet) in [(e0, e1), (e1, e0)] {
                    if let Ok(projectile_transform) = projectile_query.get(projectile) {
                        if let Ok((planet_transform, planet_momentum)) = planet_query.get(planet) {
                            let radius = mass_to_radius(planet_momentum.mass);
                            // unit vector at planet center pointing at projectile
                            let direction = (projectile_transform.translation
                                - planet_transform.translation)
                                .normalize();
                            let local_impact_site = direction * radius;
                            let event = ProjectileCollisionEvent {
                                planet,
                                projectile,
                                local_impact_site,
                            };
                            debug!("Sending projectile impact event: {event:?}");
                            projectile_collision_events.send(event);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Event)]
pub struct DespawnPlanetEvent(pub Entity);

pub fn handle_despawn_planet(
    mut commands: Commands,
    mut despawn_planet_events: EventReader<DespawnPlanetEvent>,
    projectile_query: Query<(Entity, &ProjectileTarget)>,
) {
    for &DespawnPlanetEvent(entity) in despawn_planet_events.read() {
        debug!("RECURSIVELY despawning planet {entity:?} and all of its in-flight projectiles");
        for (projectile, &ProjectileTarget { planet, .. }) in projectile_query.iter() {
            if entity == planet {
                commands.entity(projectile).despawn_recursive();
            }
        }
        commands.entity(entity).despawn_recursive();
    }
}

// FIXME: 1) this should be 'merge_planets' or something, 2) do we need to
//        "transfer" children? (explosion animation...)
pub fn transfer_planet_momentum(
    mut planet_query: Query<(&Transform, &mut Momentum, Entity)>,
    mut planet_events: EventReader<PlanetCollisionEvent>,
    mut delta_events: EventWriter<DeltaEvent>,
    mut despawn_planet_events: EventWriter<DespawnPlanetEvent>,
) {
    for PlanetCollisionEvent(e0, e1) in planet_events.read() {
        // FIXME: We have write access to `Momentum` and yet we update
        // `delta_v` via an event. Just update it here? Should `DeltaEvent`
        // even have a `delta_v` field?
        if let Ok([p0, p1]) = planet_query.get_many_mut([*e0, *e1]) {
            let (mut major, minor) = if p0.1.mass > p1.1.mass {
                (p0, p1)
            // FIXME: tie-breaker!
            } else {
                (p1, p0)
            };

            debug!("Collision of planets:");
            debug!(" Major planet {:?}", major.2);
            debug!("  position: {:?}", major.0.translation);
            debug!("  velocity: {:?}", major.1.velocity);
            debug!("  mass: {:?}", major.1.mass);
            debug!(" Minor planet {:?}", minor.2);
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
                "Directly setting mass of major planet {:?} to {combined_mass:?}",
                major.2
            );
            // Maybe increment mass via an event to?
            major.1.mass = combined_mass;
            let entity = major.2;

            let weighted_midpoint =
                ((major_factor * major.0.translation) + (minor_factor * minor.0.translation)) / 2.0;
            debug!(
                "The weighted midpoint between planets major={:?} and minor={:?} is {weighted_midpoint:?}",
                major.2, minor.2
            );
            let delta_p = weighted_midpoint - major.0.translation;
            // How much to scale in the linear (multiply major original
            // radius by this much to achieve a proportionate mass (i.e. volume) increase.
            let delta_s = major_factor.powf(-1.0 / 3.0);
            let event = DeltaEvent {
                entity,
                delta_p,
                delta_v,
                delta_s,
                force_ro: Vec3::ZERO,
            };
            debug!("Sending event: {event:?}");
            delta_events.send(event);
            debug!("Signaling despawn request for minor planet {:?}", minor.2);
            despawn_planet_events.send(DespawnPlanetEvent(minor.2));
        }
    }
}

#[derive(Bundle)]
pub struct PlanetBundle {
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
    let planet = PlanetBundle {
        pbr: PbrBundle {
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius,
                    ..default()
                })
                .unwrap(),
            ),
            material: materials.add(color.into()),
            transform: Transform::from_translation(position),
            ..default()
        },
        momentum: Momentum {
            velocity,
            mass,
            ..Default::default()
        },
        collider: Collider::ball(radius),
        ..Default::default()
    };
    let planet_id = commands.spawn(planet).id();
    debug!("Spawned planet={planet_id:?}");
}

#[derive(Component, Debug, Default)]
pub struct Momentum {
    pub velocity: Vec3,
    pub mass: f32,
    pub force_ro: Vec3,
}

#[derive(Debug, Event)]
pub struct DeltaEvent {
    pub entity: Entity,
    pub delta_p: Vec3,
    pub delta_v: Vec3,
    pub delta_s: f32,
    pub force_ro: Vec3,
}

pub fn signal_freefall_delta(
    planet_query: Query<(Entity, &Transform, &Momentum)>,
    time: Res<Time>,
    physics_config: Res<PhysicsConfig>,
    mut delta_events: EventWriter<DeltaEvent>,
) {
    let dt = time.delta_seconds();
    let mut masses = planet_query
        .iter()
        .map(|t| (t.0, t.1.translation, t.2.mass, t.2.velocity))
        .collect::<Vec<_>>();
    for _ in 0..physics_config.sims_per_frame {
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
                let delta_v = force / *mass;
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
    mut planet_query: Query<(&mut Transform, &mut Momentum)>,
    mut delta_events: EventReader<DeltaEvent>,
) {
    for event in delta_events.read() {
        if let Ok((mut transform, mut momentum)) = planet_query.get_mut(event.entity) {
            transform.translation += event.delta_p;
            momentum.velocity += event.delta_v;
            momentum.force_ro += event.force_ro;
            transform.scale *= event.delta_s;
        }
    }
}
