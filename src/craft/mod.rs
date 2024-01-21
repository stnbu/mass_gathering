use bevy::transform::TransformBundle;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use bevy_rapier3d::prelude::{
    ActiveEvents, Collider, QueryFilter, RapierContext, RigidBody, Sensor,
};

use rand::Rng;
use std::collections::HashSet;

mod controls;
pub use controls::*;

use crate::{mass_to_radius, physics::Momentum, DeltaEvent};

pub const SQRT_3: f32 = 1.7320508_f32;

#[derive(Component, PartialEq, Eq)]
pub enum SpacecraftAR {
    CrosshairsHot,
    CrosshairsCold,
}

#[derive(Debug, Default, Component)]
pub struct Spacecraft {
    pub speed: f32,
}

#[derive(Component)]
pub struct LeftCamera;

#[derive(Component)]
pub struct RightCamera;

#[derive(Resource)]
pub struct SpacecraftConfig {
    pub show_debug_markers: bool,
    pub show_impact_explosions: bool,
    pub projectile_radius: f32,
    pub stereo_enabled: bool,
    /// Hint: use a negative value for "crosseyed" mode.
    pub stereo_iod: f32, // interocular distance
    pub recoil: f32,
    pub start_transform: Transform,
    pub impact_magnitude: f32,
    pub start_speed: f32,
}

impl Default for SpacecraftConfig {
    fn default() -> Self {
        Self {
            show_debug_markers: false,
            show_impact_explosions: true,
            projectile_radius: 0.1,
            stereo_enabled: false,
            stereo_iod: 0.0,
            recoil: 0.025,
            start_transform: Default::default(),
            impact_magnitude: 25.0,
            start_speed: 0.0,
        }
    }
}

#[derive(Component)]
pub struct ProjectileTarget {
    pub planet: Entity,
    pub local_direction: Vec3,
}

#[derive(Component, Default)]
pub struct Blink {
    pub hertz: f64,
    pub start_angle: f64, // what the! not-zero seems to break.
}

#[derive(Component)]
pub struct ProjectileExplosion {
    pub rising: bool,
}

#[derive(Default)]
pub struct Despawned(HashSet<Entity>);

pub fn move_forward(mut query: Query<(&mut Transform, &Spacecraft)>, time: Res<Time>) {
    for (mut transform, spacecraft) in query.iter_mut() {
        let direction = transform.local_z();
        transform.translation -= direction * time.delta_seconds() * spacecraft.speed;
    }
}

pub fn drift(mut query: Query<&mut Transform, With<Spacecraft>>) {
    for mut transform in query.iter_mut() {
        let mut rng = rand::thread_rng();
        let rot_x = (rng.gen::<f32>() - 0.5) * 0.0003;
        let rot_y = (rng.gen::<f32>() - 0.5) * 0.0003;
        let rot_z = (rng.gen::<f32>() - 0.5) * 0.0003;
        transform.rotate(Quat::from_euler(EulerRot::XYZ, rot_x, rot_y, rot_z));
        let mov_x = (rng.gen::<f32>() - 0.5) * 0.001;
        let mov_y = (rng.gen::<f32>() - 0.5) * 0.001;
        let mov_z = (rng.gen::<f32>() - 0.5) * 0.001;
        transform.translation += Vec3::new(mov_x, mov_y, mov_z)
    }
}

pub fn spacecraft_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<SpacecraftConfig>,
) {
    let spacecraft = commands
        .spawn(TransformBundle::from_transform(config.start_transform))
        .insert(VisibilityBundle::default())
        .insert(Spacecraft {
            speed: config.start_speed,
        })
        .with_children(|child| {
            if config.stereo_enabled {
                let offset = config.stereo_iod / 2.0;
                child
                    .spawn(Camera3dBundle {
                        transform: Transform::from_xyz(offset, 0.0, 0.0),
                        ..default()
                    })
                    .insert(LeftCamera);
                child
                    .spawn(Camera3dBundle {
                        transform: Transform::from_xyz(-offset, 0.0, 0.0),
                        camera: Camera::default(),
                        camera_3d: Camera3d {
                            clear_color: ClearColorConfig::None,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(RightCamera);
            } else {
                child.spawn(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(-Vec3::Z, Vec3::Y),
                    ..default()
                });
            }
            child
                // Crosshairs "bead"
                .spawn(PbrBundle {
                    mesh: meshes.add(
                        Mesh::try_from(shape::Icosphere {
                            radius: 0.01,
                            ..Default::default()
                        })
                        .unwrap(),
                    ),
                    material: materials.add(Color::LIME_GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -7.0),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsCold);
            child
                // horizontal hair
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.005, 5.0, 0.08))),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -7.0),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsHot);
            child
                // vertical hair
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.005, 0.08))),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -6.0),
                    visibility: Visibility::Hidden,
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsHot);

            // Various lights
            child.spawn(PointLightBundle {
                transform: Transform::from_xyz(10.0, -10.0, -25.0),
                point_light: PointLight {
                    intensity: 5000.0 * 1.7,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            child.spawn(PointLightBundle {
                transform: Transform::from_xyz(-10.0, 5.0, -35.0),
                point_light: PointLight {
                    intensity: 5000.0 * 1.5,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            child.spawn(PointLightBundle {
                transform: Transform::from_xyz(30.0, -20.0, 80.0),
                point_light: PointLight {
                    intensity: 1000000.0 * 0.7,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            child.spawn(PointLightBundle {
                transform: Transform::from_xyz(-30.0, 10.0, 100.0),
                point_light: PointLight {
                    intensity: 1000000.0 * 0.8,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            child.spawn(TransformBundle::from_transform(Transform::from_xyz(
                -0.12, -0.06, -0.25,
            )));
        })
        .id();
    debug!("Spawned spacecraft with entity {spacecraft:?}");
}

pub fn set_ar_default_visibility(mut ar_query: Query<(&mut Visibility, &SpacecraftAR)>) {
    for (mut visibility, mode) in ar_query.iter_mut() {
        match mode {
            SpacecraftAR::CrosshairsCold => *visibility = Visibility::Visible,
            SpacecraftAR::CrosshairsHot => *visibility = Visibility::Hidden,
        }
    }
}

// "Hot" means the crosshairs fall on a planet. They are hidden when not on a planet.
pub fn handle_hot_planet(
    spacecraft_query: Query<&Children, With<Spacecraft>>,
    mut ar_query: Query<(&mut Visibility, &SpacecraftAR), Without<Spacecraft>>,
    hot_planet_events: EventReader<HotPlanetEvent>,
) {
    for children in spacecraft_query.iter() {
        if !hot_planet_events.is_empty() {
            for child_id in children.iter() {
                if let Ok((mut visibility, ar_element)) = ar_query.get_mut(*child_id) {
                    match *ar_element {
                        SpacecraftAR::CrosshairsHot => {
                            *visibility = Visibility::Visible;
                        }
                        SpacecraftAR::CrosshairsCold => {
                            *visibility = Visibility::Hidden;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Event)]
pub struct FireProjectileEvent;

pub fn fire_on_hot_planet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spacecraft_query: Query<&mut Transform, With<Spacecraft>>,
    config: Res<SpacecraftConfig>,
    mut hot_planet_events: EventReader<HotPlanetEvent>,
    mut fire_projectile_events: EventReader<FireProjectileEvent>,
) {
    for &HotPlanetEvent {
        planet,
        local_direction,
    } in hot_planet_events.read()
    {
        for _ in fire_projectile_events.read() {
            let mut spacecraft_transform = spacecraft_query
                .get_single_mut()
                .expect("Did not find spacecraft transform");
            debug!("Firing at planet {planet:?}, planet-local direction to target: {local_direction:?}");
            commands
                // Spawn projectile
                .spawn(PbrBundle {
                    mesh: meshes.add(
                        Mesh::try_from(shape::Icosphere {
                            radius: config.projectile_radius,
                            ..Default::default()
                        })
                        .unwrap(),
                    ),
                    material: materials.add(Color::WHITE.into()),
                    transform: Transform::from_translation(spacecraft_transform.translation),
                    ..Default::default()
                })
                // The projectile carries its "target" as a component.
                .insert(ProjectileTarget {
                    planet,
                    local_direction,
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(0.001))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Sensor);

            // Your spacecraft recoils (jiggles randomly) when you fire.
            if config.recoil != 0.0 {
                let mut rng = rand::thread_rng();
                let bump_x = (rng.gen::<f32>() - 0.5) * config.recoil;
                let bump_y = (rng.gen::<f32>() - 0.5) * config.recoil;
                let bump_z = (rng.gen::<f32>() - 0.5) * config.recoil;
                spacecraft_transform.rotate(Quat::from_euler(
                    EulerRot::XYZ,
                    bump_x,
                    bump_y,
                    bump_z,
                ));
            }
        }
    }
}

#[derive(Debug, Event)]
pub struct ProjectileCollisionEvent {
    pub planet: Entity,
    pub projectile: Entity,
    pub local_impact_site: Vec3,
}

// FIXME: need to handle in-flight projectile whose planet disappears.
pub fn handle_projectile_despawn(
    mut commands: Commands,
    mut projectile_events: EventReader<ProjectileCollisionEvent>,
) {
    for projectile_collision in projectile_events.read() {
        commands.entity(projectile_collision.projectile).despawn();
    }
}

pub fn spawn_projectile_explosion_animation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    projectile_query: Query<&ProjectileTarget>,
    planet_query: Query<&Transform, With<Momentum>>,
    mut projectile_events: EventReader<ProjectileCollisionEvent>,
) {
    for event in projectile_events.read() {
        if let Ok(projectile_target) = projectile_query.get(event.projectile) {
            if let Ok(planet_transform) = planet_query.get(event.planet) {
                // FIXME: WHY does local_impact_site need any scaling??
                let local_impact_site =
                    event.local_impact_site / (planet_transform.scale.length() / SQRT_3);
                // An explosion is a sphere centered on the planet surface at the impact site that grows and then shrinks to simulate a "fireball".
                let explosion = commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(
                            Mesh::try_from(shape::Icosphere {
                                radius: 0.2,
                                ..Default::default()
                            })
                            .unwrap(),
                        ),
                        material: materials.add(StandardMaterial {
                            base_color: Color::YELLOW,
                            perceptual_roughness: 0.99,
                            ..default()
                        }),
                        transform: Transform::from_translation(local_impact_site),
                        ..Default::default()
                    })
                    .insert(ProjectileExplosion { rising: true })
                    .id();
                commands.entity(event.planet).add_child(explosion);
                debug!(
                    "Explosion animation entity {explosion:?} spawned and now a child of planet {:?} with local coordiantes {:?}",
                    projectile_target.planet,
		    local_impact_site,
                );
            } else {
                warn!(
                    "While spawning explosion animation: planet {:?} not found",
                    event.planet
                );
            }
        } else {
            warn!(
                "While spawning explosion animation: projectile {:?} not found",
                event.projectile
            );
        }
    }
}

/// When a projectile collides with a planet, the planet's momentum is affected.
pub fn transfer_projectile_momentum(
    planet_query: Query<&Momentum, Without<ProjectileTarget>>,
    mut projectile_events: EventReader<ProjectileCollisionEvent>,
    mut delta_events: EventWriter<DeltaEvent>,
    config: Res<SpacecraftConfig>,
) {
    for event in projectile_events.read() {
        if let Ok(planet_momentum) = planet_query.get(event.planet) {
            let delta_v = -event.local_impact_site.normalize() * config.impact_magnitude
                / planet_momentum.mass;
            debug!(
                "Projectile {:?} impacting planet {:?}, delta_v={:?}",
                event.projectile, event.planet, delta_v,
            );
            delta_events.send(DeltaEvent {
                entity: event.planet,
                delta_p: Vec3::ZERO,
                delta_v,
                delta_s: 1.0,
                force_ro: Vec3::ZERO,
            });
        }
    }
}

/// Projectiles are moved toward their target planet by a constant velocity.
pub fn move_projectiles(
    mut projectile_query: Query<(Entity, &mut Transform, &ProjectileTarget)>,
    planet_query: Query<(&Transform, &mut Momentum, Entity), Without<ProjectileTarget>>,
    time: Res<Time>,
) {
    for (projectile, mut projectile_transform, target) in projectile_query.iter_mut() {
        if let Ok((planet_transform, planet_momentum, _)) = planet_query.get(target.planet) {
            let planet_radius = mass_to_radius(planet_momentum.mass);
            let target_coordinates =
                planet_transform.translation + (target.local_direction * planet_radius);
            let translation_to_target = target_coordinates - projectile_transform.translation;
            let distance = translation_to_target.length();
            let direction = translation_to_target.normalize();
            // Hardcoding the speed of a projectile.
            let speed_coefficient = 120.0;
            let absolute_velocity =
                direction * speed_coefficient * ((distance + 30.0) / (distance + 1.0));
            // constant velocity relative planet
            let velocity = absolute_velocity + planet_momentum.velocity;
            let mut translation = velocity * time.delta_seconds();
            // FIXME: If the distance to the target is less than the distance the projectile will travel, then instead just travel a bit more than that distance to ensure collision. The planet's radius should be part of this calculation.
            if translation.length() > distance {
                translation = translation_to_target * 1.1;
            }
            trace!(" Projectile {projectile:?} traveling toward target on planet {:?} by delta_p={translation:?}", target.planet);
            projectile_transform.translation += translation;
        } else {
            warn!(
                "While moving projectile: planet {:?} not found",
                target.planet
            );
        }
    }
}

/// Animate the "explosion" of the collision.
pub fn animate_projectile_explosion(
    mut commands: Commands,
    mut explosion_query: Query<(Entity, &mut Transform, &mut ProjectileExplosion)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut explosion) in explosion_query.iter_mut() {
        let animation_direction = if explosion.rising { 3.5 } else { -2.0 };
        transform.scale += Vec3::splat(1.0) * 0.2 * animation_direction * time.delta_seconds();
        if transform.scale.length() > 3.0 {
            explosion.rising = false;
        }
        let mut coords = [0.0; 3];
        transform.scale.write_to_slice(&mut coords);
        // A decaying explosion is scaled down in each frame. When any of the coordinates become negative (scaled downward past zero), then the explosion entity is despawned.
        for d in coords {
            if d < 0.0 {
                debug!("despawning explosion entity {:?}", entity);
                commands.entity(entity).despawn();
                return;
            }
        }
    }
}

#[derive(Event)]
pub struct HotPlanetEvent {
    pub planet: Entity,
    // This is: the direction to the impact site relative to the planet's transform
    pub local_direction: Vec3,
}

pub fn signal_hot_planet(
    planet_query: Query<&Transform, With<Momentum>>,
    spacecraft_query: Query<&Transform, With<Spacecraft>>,
    rapier_context: Res<RapierContext>,
    mut hot_planet_events: EventWriter<HotPlanetEvent>,
) {
    for pov in spacecraft_query.iter() {
        let ray_origin = pov.translation;
        let ray_direction = -1.0 * pov.local_z();
        let intersection = rapier_context.cast_ray(
            ray_origin,
            ray_direction,
            150.0, // Beyond this distance, the crosshairs are inactive.
            true,
            QueryFilter::only_dynamic(),
        );

        if let Some((planet, distance)) = intersection {
            if let Ok(planet_transform) = planet_query.get(planet) {
                let global_impact_site = ray_origin + (ray_direction * distance);
                let local_direction =
                    (global_impact_site - planet_transform.translation).normalize();
                let event = HotPlanetEvent {
                    planet,
                    local_direction,
                };
                hot_planet_events.send(event);
            }
        }
    }
}
