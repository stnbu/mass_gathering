use bevy::transform::TransformBundle;
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::camera::Viewport,
    window::{WindowId, WindowResized},
};
use bevy_egui::{
    egui::{
        style::Margin, Color32, FontFamily::Monospace, FontId, Frame, RichText, TopBottomPanel,
    },
    EguiContext,
};
use bevy_rapier3d::prelude::{
    ActiveEvents, Collider, CollisionEvent, QueryFilter, RapierContext, RigidBody, Sensor,
};
use std::f64::consts::PI;

use std::collections::HashSet;
use std::f32::consts::TAU;
use std::time::Duration;

use crate::physics::{mass_to_radius, Momentum};

#[derive(Component, PartialEq, Eq)]
pub enum SpacecraftAR {
    CrosshairsHot,
    CrosshairsCold,
}

#[derive(Debug, Default, Component)]
pub struct Spacecraft {
    gain: Vec3,
    pub speed: f32,
    pub hot_planet: Option<Entity>,
}

#[derive(Component)]
pub struct LeftCamera;

#[derive(Component)]
pub struct RightCamera;

pub struct SpacecraftConfig {
    pub show_debug_markers: bool,
    pub show_impact_explosions: bool,
    pub projectile_radius: f32,
    pub stereo_enabled: bool,
    /// Hint: use a netative value for "crosseyed" mode.
    pub stereo_iod: f32, // interocular distance
}

impl Default for SpacecraftConfig {
    fn default() -> Self {
        Self {
            show_debug_markers: false,
            show_impact_explosions: true,
            projectile_radius: 0.1,
            stereo_enabled: false,
            stereo_iod: 0.0,
        }
    }
}

#[derive(Component)]
pub struct DespawnTimer {
    pub ttl: Timer,
}

#[derive(Component)]
pub struct BallisticProjectileTarget {
    pub planet: Entity,
    pub local_impact_site: Vec3,
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

pub fn timer_despawn(
    mut commands: Commands,
    mut despawn_query: Query<(Entity, &mut DespawnTimer)>,
    time: Res<Time>,
) {
    for (entity, mut despawn_timer) in despawn_query.iter_mut() {
        despawn_timer.ttl.tick(time.delta());
        if despawn_timer.ttl.finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct DelayedVisibility {
    pub timer: Timer,
}

pub fn delayed_visibility(
    mut delayed_visibility_query: Query<(&mut Visibility, &mut DelayedVisibility)>,
    time: Res<Time>,
) {
    for (mut visibility, mut delay) in delayed_visibility_query.iter_mut() {
        delay.timer.tick(time.delta());
        if delay.timer.finished() {
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

pub fn move_forward(mut query: Query<(&mut Transform, &Spacecraft)>, time: Res<Time>) {
    for (mut transform, spacecraft) in query.iter_mut() {
        let direction = transform.local_z();
        transform.translation -= direction * time.delta_seconds() * spacecraft.speed;
    }
}

pub fn steer(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &mut Spacecraft)>) {
    let gain = 0.2;
    let nudge = TAU / 10000.0;
    let mut roll = 0.0;
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let mut had_input = false;

    let (mut transform, mut spacecraft) = query.get_single_mut().unwrap();

    // `just_presssed` ignores keys held down.
    for key in keys.get_just_pressed() {
        match key {
            KeyCode::PageUp => {
                spacecraft.speed += 1.0 + spacecraft.speed * 0.05;
            }
            KeyCode::PageDown => {
                spacecraft.speed -= 1.0 + spacecraft.speed * 0.05;
            }
            _ => {}
        }
    }

    // Make it easier to find "neutral"
    if spacecraft.speed.abs() < 0.5 {
        spacecraft.speed = 0.0
    }

    // `presssed` (contrast `just_pressed`) considers keys being _held_ down, which is good for rotation controls.
    for key in keys.get_pressed() {
        had_input = true;
        match key {
            KeyCode::Left => {
                yaw += nudge * (spacecraft.gain.z + 1.0);
                spacecraft.gain.z += gain;
            }
            KeyCode::Right => {
                yaw -= nudge * (spacecraft.gain.z + 1.0);
                spacecraft.gain.z += gain;
            }
            KeyCode::Up => {
                pitch += nudge * (spacecraft.gain.x + 1.0);
                spacecraft.gain.x += gain;
            }
            KeyCode::Down => {
                pitch -= nudge * (spacecraft.gain.x + 1.0);
                spacecraft.gain.x += gain;
            }
            KeyCode::Z => {
                roll += nudge * (spacecraft.gain.y + 1.0);
                spacecraft.gain.y += gain;
            }
            KeyCode::X => {
                roll -= nudge * (spacecraft.gain.y + 1.0);
                spacecraft.gain.y += gain;
            }
            _ => {
                had_input = false;
            }
        }
    }

    if !had_input {
        if spacecraft.gain.x > 0.0 {
            spacecraft.gain.x -= gain;
            if spacecraft.gain.x < 0.0 {
                spacecraft.gain.x = 0.0;
            }
        }
        if spacecraft.gain.y > 0.0 {
            spacecraft.gain.y -= gain;
            if spacecraft.gain.y < 0.0 {
                spacecraft.gain.y = 0.0;
            }
        }
        if spacecraft.gain.z > 0.0 {
            spacecraft.gain.z -= gain;
            if spacecraft.gain.z < 0.0 {
                spacecraft.gain.z = 0.0;
            }
        }
    }

    if roll != 0.0 || pitch != 0.0 || yaw != 0.0 {
        let local_x = transform.local_x();
        let local_y = transform.local_y();
        let local_z = transform.local_z();
        transform.rotate(Quat::from_axis_angle(local_x, pitch));
        transform.rotate(Quat::from_axis_angle(local_z, roll));
        transform.rotate(Quat::from_axis_angle(local_y, yaw));
    }
}

pub fn spacecraft_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<SpacecraftConfig>,
) {
    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_xyz(0.0, 0.0, 40.0).looking_at(-Vec3::Z, Vec3::Y),
        ))
        .insert_bundle(VisibilityBundle::default())
        .insert(Spacecraft::default())
        .with_children(|parent| {
            if config.stereo_enabled {
                let offset = config.stereo_iod / 2.0;
                parent
                    .spawn_bundle(Camera3dBundle {
                        transform: Transform::from_xyz(offset, 0.0, 0.0),
                        ..default()
                    })
                    .insert(LeftCamera);
                parent
                    .spawn_bundle(Camera3dBundle {
                        transform: Transform::from_xyz(-offset, 0.0, 0.0),
                        camera: Camera {
                            priority: 1,
                            ..default()
                        },
                        camera_3d: Camera3d {
                            clear_color: ClearColorConfig::None,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(RightCamera);
            } else {
                parent.spawn_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(-Vec3::Z, Vec3::Y),
                    ..default()
                });
            }
            // Possibly the worst way to implement "crosshairs" evar.
            parent
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.03,
                        ..Default::default()
                    })),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -8.0),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsCold);
            parent
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.005, 5.0, 0.1))),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -7.0),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsHot);
            parent
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.005, 0.1))),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -6.0),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsHot);

            // Various lights for seeing
            parent.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(10.0, -10.0, -25.0),
                point_light: PointLight {
                    intensity: 5000.0 * 1.7,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(-10.0, 5.0, -35.0),
                point_light: PointLight {
                    intensity: 5000.0 * 1.5,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(30.0, -20.0, 80.0),
                point_light: PointLight {
                    intensity: 1000000.0 * 0.7,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(-30.0, 10.0, 100.0),
                point_light: PointLight {
                    intensity: 1000000.0 * 0.8,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.08, 0.08, -1.0))),
            material: materials.add(Color::GREEN.into()),
            ..Default::default()
        })
        .insert(MomentumVector);
}

pub fn do_blink(mut blinker_query: Query<(&mut Visibility, &Blink)>, time: Res<Time>) {
    for (mut visibility, blink_config) in blinker_query.iter_mut() {
        assert!(blink_config.start_angle == 0.0, "This does not work.");
        let period = 1.0 / blink_config.hertz;
        let portion = blink_config.start_angle / (2.0 * PI);
        let offset = portion * period;
        let elapsed = time.seconds_since_startup() + offset;
        let whole_cycles_elapsed = (elapsed / period).trunc();
        let until_next_cycle = elapsed - (whole_cycles_elapsed * period) + offset;
        if until_next_cycle < 1.0 / 60.1 {
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

pub fn set_ar_default_visibility(
    mut crosshairs_query: Query<(&mut Visibility, &SpacecraftAR), Without<MomentumVector>>,

    mut the_momentum_vector: Query<&mut Visibility, With<MomentumVector>>,
) {
    the_momentum_vector.get_single_mut().unwrap().is_visible = false;
    for (mut visibility, mode) in crosshairs_query.iter_mut() {
        match mode {
            SpacecraftAR::CrosshairsCold => visibility.is_visible = true,
            SpacecraftAR::CrosshairsHot => visibility.is_visible = false,
        }
    }
}

/*

                           if let Ok((transform, momentum)) = planet_query.get(planet_id) {
                               let radius = mass_to_radius(momentum.mass);
                               let momentum = momentum.velocity * momentum.mass;
                               *element_transform = Transform {
                                   translation: transform.translation
                                       + Vec3::new(0.0, 0.0, -(2.5 + radius)),
                                   rotation: Quat::from_rotation_arc(
                                       Vec3::Z,
                                       momentum.normalize(),
                                   ),
                                   ..default()
                               };
                               visibility.is_visible = true;


*/

#[derive(Component)]
pub struct MomentumVector;
pub fn handle_hot_planet(
    spacecraft_query: Query<
        (&Children, &Spacecraft),
        (Without<SpacecraftAR>, Without<MomentumVector>),
    >,
    mut planet_query: Query<
        (&Transform, &Momentum),
        (Without<SpacecraftAR>, Without<MomentumVector>),
    >,
    mut ar_query: Query<(&mut Visibility, &mut Transform, &SpacecraftAR), Without<MomentumVector>>,
    mut the_momentum_vector: Query<(&mut Transform, &mut Visibility), With<MomentumVector>>,
) {
    // FIXME -- Gets hairy when multiple "spacecraft". We want only _our_ markup to be visible.
    for (children, spacecraft) in spacecraft_query.iter() {
        if let Some(planet_id) = spacecraft.hot_planet {
            if let Ok((transform, momentum)) = planet_query.get_mut(planet_id) {
                let radius = mass_to_radius(momentum.mass);
                //let momentum_vec3 = momentum.velocity * momentum.mass;
                let (mut momentum_vector_transform, mut visibility) =
                    the_momentum_vector.get_single_mut().unwrap();

                visibility.is_visible = true;
                let magnitude = (momentum.velocity * momentum.mass).length();
                *momentum_vector_transform = Transform {
                    translation: transform.translation
                        + momentum.velocity.normalize() * ((magnitude / 2.0) + radius),
                    rotation: Quat::from_rotation_arc(-Vec3::Z, momentum.velocity.normalize()),
                    scale: Vec3::new(1.0, 1.0, magnitude),
                };
            }
            for child_id in children.iter() {
                if let Ok((mut visibility, mut element_transform, ar_element)) =
                    ar_query.get_mut(*child_id)
                {
                    match *ar_element {
                        SpacecraftAR::CrosshairsHot => {
                            visibility.is_visible = true;
                        }
                        SpacecraftAR::CrosshairsCold => {
                            visibility.is_visible = false;
                        }
                    }
                }
            }
        }
    }
}

/*
Query<&mut Visibility, With<PlanetMarkup>> in systemhandle_hot_planet accesses component(s) Visibility in a way that conflicts with a previous system parameter. Consider using `Without<T>` to create disjoint Queries or merging conflicting Queries into a `ParamSet`.', /Users/mburr/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/system/system_param.rs:205:5
*/

pub fn handle_projectile_engagement(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    optional_keys: Option<Res<Input<KeyCode>>>,
    planet_query: Query<
        (Entity, &Transform),
        (
            Without<BallisticProjectileTarget>,
            With<Momentum>,
            With<Collider>,
        ),
    >,
    rapier_context: Res<RapierContext>,
    mut spacecraft_query: Query<(&Transform, &mut Spacecraft)>,
    config: Res<SpacecraftConfig>,
) {
    for (pov, mut spacecraft) in spacecraft_query.iter_mut() {
        let ray_origin = pov.translation - pov.local_y() * config.projectile_radius * 1.2;
        let ray_direction = -1.0 * pov.local_z();
        let intersection = rapier_context.cast_ray(
            ray_origin,
            ray_direction,
            150.0, // what's reasonable here...?
            true,
            QueryFilter::only_dynamic(),
        );

        if let Some((intersected_collider_id, distance)) = intersection {
            let (planet_id, planet_transform) =
                if let Ok(result) = planet_query.get(intersected_collider_id) {
                    result
                } else {
                    debug!("No planet found with id {intersected_collider_id:?}.");
                    continue;
                };
            spacecraft.hot_planet = Some(planet_id);
            if let Some(ref keys) = optional_keys {
                if keys.just_pressed(KeyCode::F) {
                    debug!("Firing projectile!");
                    let scale_factor = planet_transform.scale.length();
                    let global_impact_site = ray_origin + (ray_direction * distance);
                    let local_impact_site = global_impact_site - planet_transform.translation;
                    if config.show_debug_markers {
                        let planet_local_marker = commands
                            .spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Icosphere {
                                    radius: 0.15,
                                    ..Default::default()
                                })),
                                material: materials.add(Color::BLUE.into()),
                                transform: Transform::from_translation(
                                    local_impact_site / (scale_factor / 1.7), // yeah
                                ),
                                ..Default::default()
                            })
                            .insert(Blink {
                                hertz: 4.9,
                                ..default()
                            })
                            .insert(DespawnTimer {
                                ttl: Timer::new(Duration::from_secs(500), false),
                            })
                            .id();
                        commands.entity(planet_id).add_child(planet_local_marker);
                        //global marker (should diverge as planet moves)
                        let global_marker = commands
                            .spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Icosphere {
                                    radius: 0.15,
                                    ..Default::default()
                                })),
                                material: materials.add(Color::RED.into()),
                                transform: Transform::from_translation(global_impact_site),
                                ..Default::default()
                            })
                            .insert(Blink {
                                hertz: 5.1,
                                ..default()
                            })
                            .insert(DespawnTimer {
                                ttl: Timer::new(Duration::from_secs(500), false),
                            })
                            .id();
                        debug!("Placing two debug markers. global={global_marker:?} and local={planet_local_marker:?}[parent={planet_id:?}]");
                    }
                    let radius = config.projectile_radius;
                    let projectile = commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere {
                                radius,
                                ..Default::default()
                            })),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::from_translation(ray_origin),
                            ..Default::default()
                        })
                        .insert(BallisticProjectileTarget {
                            planet: planet_id,
                            local_impact_site,
                        })
                        .insert(RigidBody::Dynamic)
                        .insert(Collider::ball(radius))
                        .insert(ActiveEvents::COLLISION_EVENTS)
                        .insert(Sensor)
                        .id();
                    debug!("Projectile entity: {projectile:?}");
                    debug!("-	ray_origin={ray_origin:?}");
                    debug!("-	ray_direction={ray_direction:?}");
                    debug!("-	global_impact_site={global_impact_site:?}");
                    debug!("-	local_impact_site={local_impact_site:?}");
                }
            }
        } else {
            spacecraft.hot_planet = None;
        }
    }
}

// FIXME -- it may make more sense for the "target" to be a _child_ of the target planet.
pub fn handle_projectile_flight(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut projectile_query: Query<(Entity, &mut Transform, &BallisticProjectileTarget)>,
    planet_query: Query<(&Transform, &Momentum, Entity), Without<BallisticProjectileTarget>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut despawned: Local<Despawned>,
    time: Res<Time>,
    config: Res<SpacecraftConfig>,
) {
    let mut collided = HashSet::new();
    for event in collision_events.iter() {
        if let CollisionEvent::Started(e0, e1, _) = event {
            collided.insert(e0);
            collided.insert(e1);
        }
    }
    for (projectile, mut projectile_transform, target) in projectile_query.iter_mut() {
        // FIXME -- ensure that target.planet is still there
        if despawned.0.contains(&projectile) {
            warn!("We already despawned {:?}", projectile);
            continue;
        }
        debug!(
            "Handling flight of projectile {projectile:?} with target {:?}",
            target.planet
        );
        if collided.contains(&projectile) {
            debug!("We have a collision start event for projectile {projectile:?}.");
            if config.show_impact_explosions {
                // FIXME -- the fact that we are getting the target planet here and also
                //          below where we move the planet...says we need refactoring.
                if let Ok((planet_transform, _, _)) = planet_query.get(target.planet) {
                    let scale_factor = planet_transform.scale.length();
                    let explosion = commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere {
                                radius: 0.2,
                                ..Default::default()
                            })),
                            material: materials.add(StandardMaterial {
                                base_color: Color::YELLOW,
                                perceptual_roughness: 0.99,
                                ..default()
                            }),
                            transform: Transform::from_translation(
                                target.local_impact_site / (scale_factor / 1.7),
                            ),
                            ..Default::default()
                        })
                        .insert(ProjectileExplosion { rising: true })
                        .id();
                    debug!("Explosion animation entity: {explosion:?}");
                    commands.entity(target.planet).add_child(explosion);
                }
            }
            debug!("Despawning projectile entity {projectile:?}");
            commands.entity(projectile).despawn();
            despawned.0.insert(projectile);
            continue;
        } else if !collided.is_empty() {
            debug!(
                "Projectile {projectile:?} not in collision entity list: {:?}",
                collided
            );
        }
        if let Ok((planet_transform, planet_momentum, _)) = planet_query.get(target.planet) {
            let goal_impact_site = planet_transform.translation + target.local_impact_site;
            let direction = (projectile_transform.translation - goal_impact_site).normalize();
            let translation =
                (direction + (planet_momentum.velocity * time.delta_seconds() * 0.8)) * 0.4;
            projectile_transform.translation -= translation;
            debug!(
                "Projectile entity {projectile:?} traveled {:?}",
                translation.length()
            );
        } else {
            debug!(
                "No projectile movement. Target planet {:?} not in collider query contents: {:?}",
                target.planet,
                planet_query.iter().map(|(_, _, e)| e).collect::<Vec<_>>()
            );
        }
    }
}

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
        for d in coords {
            if d < 0.0 {
                debug!("despawning explosion entity {:?}", entity);
                commands.entity(entity).despawn();
                return;
            }
        }
    }
}

pub fn hud(
    mut ctx: ResMut<EguiContext>,
    query: Query<(&Spacecraft, &Transform)>,
    config: Res<SpacecraftConfig>,
) {
    // FIXME vvv
    if config.stereo_enabled {
        return;
    }
    let (spacecraft, transform) = query.get_single().unwrap();
    let hud_text = format!(
        " [ NOTE CHANGES ]
Arrow Keys - Pitch & Yaw
Z & X      - Roll
PgUp/PgDn  - Speed
F          - Fire

Your Speed - {}
Your Location
  x        - {}
  y        - {}
  z        - {}
",
        spacecraft.speed, transform.translation.x, transform.translation.y, transform.translation.z
    );

    TopBottomPanel::top("hud")
        .frame(Frame {
            outer_margin: Margin::symmetric(10.0, 20.0),
            fill: Color32::TRANSPARENT,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.label(RichText::new(hud_text).color(Color32::GREEN).font(FontId {
                size: 18.0,
                family: Monospace,
            }));
        });
}

pub fn set_camera_viewports(
    windows: Res<Windows>,
    mut resize_events: EventReader<WindowResized>,
    mut left_camera: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
    mut right_camera: Query<&mut Camera, (With<RightCamera>, Without<LeftCamera>)>,
    config: Res<SpacecraftConfig>,
) {
    // FIXME vvv
    if !config.stereo_enabled {
        return;
    }
    for resize_event in resize_events.iter() {
        if resize_event.id == WindowId::primary() {
            let window = windows.primary();

            let mut left_viewport = left_camera.single_mut();
            let mut right_viewport = right_camera.single_mut();

            left_viewport.viewport = Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(window.physical_width() / 2, window.physical_height()),
                ..default()
            });

            right_viewport.viewport = Some(Viewport {
                physical_position: UVec2::new(window.physical_width() / 2, 0),
                physical_size: UVec2::new(window.physical_width() / 2, window.physical_height()),
                ..default()
            });
        }
    }
}
