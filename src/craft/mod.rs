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
    ActiveEvents, Collider, QueryFilter, RapierContext, RigidBody, Sensor,
};

use rand::Rng;
use std::collections::HashSet;
use std::f32::consts::TAU;
use std::f64::consts::PI;

use crate::mass_to_radius;
use crate::physics::Momentum;

pub const SQRT_3: f32 = 1.7320508_f32;

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
    /// Hint: use a negative value for "crosseyed" mode.
    pub stereo_iod: f32, // interocular distance
    pub recoil: f32,
    pub start_transform: Transform,
    pub impact_magnitude: f32,
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
            KeyCode::A => {
                yaw += nudge * (spacecraft.gain.z + 1.0);
                spacecraft.gain.z += gain;
            }
            KeyCode::D => {
                yaw -= nudge * (spacecraft.gain.z + 1.0);
                spacecraft.gain.z += gain;
            }
            KeyCode::W => {
                pitch += nudge * (spacecraft.gain.x + 1.0);
                spacecraft.gain.x += gain;
            }
            KeyCode::S => {
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
        .spawn_bundle(TransformBundle::from_transform(config.start_transform))
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
                        radius: 0.01,
                        ..Default::default()
                    })),
                    material: materials.add(Color::LIME_GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -7.0),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsCold);
            parent
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.005, 5.0, 0.08))),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_xyz(0.0, 0.0, -7.0),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(SpacecraftAR::CrosshairsHot);
            parent
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.005, 0.08))),
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
    mut ar_query: Query<(&mut Visibility, &SpacecraftAR), Without<Breadcrumb>>,
    mut breadcrumb_query: Query<&mut Visibility, With<Breadcrumb>>,
) {
    debug!("Setting default visibility for AR components");
    for (mut visibility, mode) in ar_query.iter_mut() {
        debug!("    .");
        match mode {
            SpacecraftAR::CrosshairsCold => visibility.is_visible = true,
            SpacecraftAR::CrosshairsHot => visibility.is_visible = false,
        }
    }
    for mut breadcrumb in breadcrumb_query.iter_mut() {
        breadcrumb.is_visible = false;
    }
}

use crate::physics::Breadcrumb;
use crate::prelude::DeltaEvent;
pub fn handle_hot_planet(
    spacecraft_query: Query<(&Children, &Spacecraft)>,
    mut ar_query: Query<
        (Entity, &mut Visibility, &SpacecraftAR),
        (Without<Spacecraft>, Without<Breadcrumb>),
    >,
    mut breadcrumb_query: Query<(&mut Visibility, &Breadcrumb)>,
) {
    for (children, spacecraft) in spacecraft_query.iter() {
        if let Some(planet) = spacecraft.hot_planet {
            debug!("Planet hot: {planet:?}");
            for child_id in children.iter() {
                if let Ok((id, mut visibility, ar_element)) = ar_query.get_mut(*child_id) {
                    debug!(
                        "  Setting visibility for crosshairs child component {id:?} for hot planet {planet:?}"
                    );
                    match *ar_element {
                        SpacecraftAR::CrosshairsHot => {
                            debug!("    Showing hot component");
                            visibility.is_visible = true;
                        }
                        SpacecraftAR::CrosshairsCold => {
                            debug!("    Hiding cold component");
                            visibility.is_visible = false;
                        }
                    }
                }
            }
            for (mut visibility, breadcrumb) in breadcrumb_query.iter_mut() {
                if planet == breadcrumb.0 {
                    visibility.is_visible = true;
                }
            }
        }
    }
}

pub fn handle_projectile_engagement(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    optional_keys: Option<Res<Input<KeyCode>>>,
    planet_query: Query<
        (Entity, &Transform),
        (
            Without<ProjectileTarget>,
            With<Momentum>,
            With<Collider>,
            Without<Spacecraft>,
        ),
    >,
    rapier_context: Res<RapierContext>,
    mut spacecraft_query: Query<(&mut Transform, &mut Spacecraft)>,
    config: Res<SpacecraftConfig>,
) {
    for (mut pov, mut spacecraft) in spacecraft_query.iter_mut() {
        let ray_origin = pov.translation; // - pov.local_y() * config.projectile_radius * 1.2;
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
                if keys.just_pressed(KeyCode::Space) {
                    let global_impact_site = ray_origin + (ray_direction * distance);
                    let local_direction =
                        (global_impact_site - planet_transform.translation).normalize();
                    debug!("Firing at planet {planet_id:?}, planet-local direction to target: {local_direction:?}");
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Icosphere {
                                radius: config.projectile_radius,
                                ..Default::default()
                            })),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::from_translation(ray_origin),
                            ..Default::default()
                        })
                        .insert(ProjectileTarget {
                            planet: planet_id,
                            // go to here?
                            local_direction,
                        })
                        .insert(RigidBody::Dynamic)
                        .insert(Collider::ball(0.001))
                        .insert(ActiveEvents::COLLISION_EVENTS)
                        .insert(Sensor);
                    // // Add some recoil excitement
                    // if config.recoil != 0.0 {
                    //     let mut rng = rand::thread_rng();
                    //     let bump_x = (rng.gen::<f32>() - 0.5) * config.recoil;
                    //     let bump_y = (rng.gen::<f32>() - 0.5) * config.recoil;
                    //     let bump_z = (rng.gen::<f32>() - 0.5) * config.recoil;
                    //     pov.rotate(Quat::from_euler(EulerRot::XYZ, bump_x, bump_y, bump_z));
                    // }
                }
            }
        } else {
            spacecraft.hot_planet = None;
        }
    }
}

#[derive(Debug)]
pub struct ProjectileCollisionEvent {
    pub planet: Entity,
    pub projectile: Entity,
    pub local_impact_site: Vec3,
}

// WARNING: order matters
pub fn handle_projectile_despawn(
    mut commands: Commands,
    mut projectile_events: EventReader<ProjectileCollisionEvent>,
) {
    for projectile_collision in projectile_events.iter() {
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
    for event in projectile_events.iter() {
        warn!("[spawn_projectile_explosion_animation] Receiving projectile collision event: {event:?}");
        if let Ok(projectile_target) = projectile_query.get(event.projectile) {
            if let Ok(planet_transform) = planet_query.get(event.planet) {
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
                            event.local_impact_site / (planet_transform.scale.length() / SQRT_3),
                        ),
                        ..Default::default()
                    })
                    .insert(ProjectileExplosion { rising: true })
                    .id();
                commands.entity(event.planet).add_child(explosion);
                warn!(
                    "Explosion animation entity {explosion:?} spawned and now a child of planet {:?} with local coordiantes {:?}",
                    projectile_target.planet,
		    event.local_impact_site,
                );
            } else {
                warn!(
                    "[spawn_projectile_explosion_animation] Did not find planet {:?}",
                    event.planet
                );
            }
        } else {
            // FIXME: should be possible to guarantee this never happens.
            warn!(
                "While spawning explosion animation: projectile {:?} not found",
                event.projectile
            );
        }
    }
}

pub fn transfer_projectile_momentum(
    planet_query: Query<&Momentum, Without<ProjectileTarget>>,
    mut projectile_events: EventReader<ProjectileCollisionEvent>,
    mut delta_events: EventWriter<DeltaEvent>,
    config: Res<SpacecraftConfig>,
) {
    for event in projectile_events.iter() {
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
            });
        }
    }
}

pub fn move_projectiles(
    mut projectile_query: Query<(Entity, &mut Transform, &ProjectileTarget)>,
    planet_query: Query<(&Transform, &mut Momentum, Entity), Without<ProjectileTarget>>,
    time: Res<Time>,
) {
    for (projectile, mut projectile_transform, target) in projectile_query.iter_mut() {
        debug!(
            "Handling flight of projectile {projectile:?} with target {:?}",
            target.planet
        );
        if let Ok((planet_transform, planet_momentum, _)) = planet_query.get(target.planet) {
            let planet_radius = mass_to_radius(planet_momentum.mass);
            let target = planet_transform.translation + (target.local_direction * planet_radius);
            let translation_to_target = target - projectile_transform.translation;
            let distance = translation_to_target.length();
            let direction = translation_to_target.normalize();

            let speed_coefficient = 0.32 * 10.0 * 50.0 * 0.75;
            let absolute_velocity =
                direction * speed_coefficient * ((distance + 30.0) / (distance + 1.0));
            // constant velocity relative planet
            let velocity = absolute_velocity + planet_momentum.velocity;
            let mut translation = velocity * time.delta_seconds();
            if translation.length() > distance {
                // NOTE: We stretch the translation_to_target by a tiny bit to ensure we get a collision.
                // Testing reveals: If the final step is exactly traslation_to_target, there is never
                // a collision (and the projectile is "moved" in the next frame by `Vec3(NaN, NaN, NaN)`).
                let final_translation = translation_to_target;
                debug!(" Next projectile translation larger than distance to target. Resetting to to-target translation vector: {translation_to_target:?}");
                translation = final_translation * 1.1;
            }
            debug!(" Projectile traveling delta_p={translation:?}");
            projectile_transform.translation += translation;
        } else {
            warn!(
                "Target planet {:?} despawned before projectile impact.",
                target.planet
            );
            // ...
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
        "\
W & S      - Pitch
A & D      - Yaw
Z & X      - Roll
PgUp/PgDn  - Speed
Space      - Fire
P          - Pause
------------------
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
