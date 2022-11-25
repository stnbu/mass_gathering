use bevy::prelude::*;

use bevy_rapier3d::prelude::{
    ActiveEvents, Collider, QueryFilter, RapierContext, RigidBody, Sensor,
};

use std::collections::HashSet;

mod controls;
pub use controls::*;

mod setup;
pub use setup::*;

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
    pub speed: f32,
}

#[derive(Component)]
pub struct LeftCamera;

#[derive(Component)]
pub struct RightCamera;

pub struct SpacecraftConfig {
    pub show_impact_explosions: bool,
    pub projectile_radius: f32,
    /// Hint: use a negative value for "crosseyed" mode.
    pub start_transform: Transform,
    pub impact_magnitude: f32,
    pub start_speed: f32,
}

impl Default for SpacecraftConfig {
    fn default() -> Self {
        Self {
            show_impact_explosions: true,
            projectile_radius: 0.1,
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

const BALL_RADIUS: f32 = 3.5 / 14.0;
const FLOAT_HEIGHT: f32 = 2.0 / 14.0;
const CYLINDER_RADIUS: f32 = 1.0 / 14.0;
const CONE_HEIGHT: f32 = 2.0 / 14.0;
const CONE_RADIUS: f32 = 2.0 / 14.0;

use crate::mg_shapes::*;

#[derive(Component)]
pub struct VectorBallTransform;

pub fn set_ar_default_visibility(mut ar_query: Query<(&mut Visibility, &SpacecraftAR)>) {
    for (mut visibility, mode) in ar_query.iter_mut() {
        match mode {
            SpacecraftAR::CrosshairsCold => visibility.is_visible = true,
            SpacecraftAR::CrosshairsHot => visibility.is_visible = false,
        }
    }
}

use crate::prelude::DeltaEvent;
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
    } in hot_planet_events.iter()
    {
        for _ in fire_projectile_events.iter() {
            let spacecraft_transform = spacecraft_query.get_single_mut().unwrap();
            debug!("Firing at planet {planet:?}, planet-local direction to target: {local_direction:?}");
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: config.projectile_radius,
                        ..Default::default()
                    })),
                    material: materials.add(Color::WHITE.into()),
                    transform: Transform::from_translation(spacecraft_transform.translation),
                    ..Default::default()
                })
                .insert(ProjectileTarget {
                    planet,
                    local_direction,
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(0.001)) // FIXME: does size matter?
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Sensor);
        }
    }
}

#[derive(Debug)]
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
        if let Ok(projectile_target) = projectile_query.get(event.projectile) {
            if let Ok(planet_transform) = planet_query.get(event.planet) {
                // FIXME: WHY does local_impact_site need any scaling??
                let local_impact_site =
                    event.local_impact_site / (planet_transform.scale.length() / SQRT_3);
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
                force_ro: Vec3::ZERO,
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
        if let Ok((planet_transform, planet_momentum, _)) = planet_query.get(target.planet) {
            let planet_radius = mass_to_radius(planet_momentum.mass);
            let target_coordinates =
                planet_transform.translation + (target.local_direction * planet_radius);
            let translation_to_target = target_coordinates - projectile_transform.translation;
            let distance = translation_to_target.length();
            let direction = translation_to_target.normalize();

            let speed_coefficient = 120.0;
            // FIXME: Tweak, experiment with inverse distance acceleration.
            let absolute_velocity =
                direction * speed_coefficient * ((distance + 30.0) / (distance + 1.0));
            // constant velocity relative planet
            let velocity = absolute_velocity + planet_momentum.velocity;
            let mut translation = velocity * time.delta_seconds();
            if translation.length() > distance {
                // FIXME: this "works" but it needs invesgation. Do we need it?
                // shouldn't it be a function of radius?
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
            150.0, // what's reasonable here...?
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

pub struct VectorBallUpdate {
    element: VectorBallElement,
    vector: Vec3,
    origin: Vec3,
}

pub struct VectorParts {
    cylinder: Entity,
    cone: Entity,
}
use std::collections::HashMap;
pub struct VectorBallData {
    pub ball: Option<Entity>,
    pub vectors: HashMap<VectorBallElement, VectorParts>,
    pub scale: f32,
}

impl Default for VectorBallData {
    fn default() -> Self {
        Self {
            ball: None,
            vectors: HashMap::new(),
            scale: 0.02,
        }
    }
}

pub fn create_vector_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut vector_ball_data: ResMut<VectorBallData>,
) {
    let material = StandardMaterial {
        base_color: Color::GREEN,
        //emissive: Color::rgba_linear(0.0, 0.0, 0.0, 0.0),
        reflectance: 0.005,
        ..default()
    };

    let ball = commands
        .spawn_bundle(PbrBundle {
            visibility: Visibility { is_visible: false },
            mesh: meshes.add(
                (shape::Icosphere {
                    radius: BALL_RADIUS,
                    ..Default::default()
                })
                .into(),
            ),
            material: materials.add(material.clone()),
            ..Default::default()
        })
        .insert(VectorBallElement::Ball)
        .id();
    vector_ball_data.ball = Some(ball);

    [VectorBallElement::Momentum].iter().for_each(|element| {
        let cone = commands
            .spawn_bundle(PbrBundle {
                visibility: Visibility { is_visible: false },
                mesh: meshes.add(
                    (Cone {
                        radius: CONE_RADIUS,
                        height: CONE_HEIGHT,
                        ..Default::default()
                    })
                    .into(),
                ),
                material: materials.add(material.clone()),
                ..Default::default()
            })
            .insert(*element)
            .id();
        let cylinder = commands
            .spawn_bundle(PbrBundle {
                visibility: Visibility { is_visible: false },
                mesh: meshes.add(
                    (Cylinder {
                        height: 1.0,
                        radius_bottom: CYLINDER_RADIUS,
                        radius_top: CYLINDER_RADIUS,
                        ..Default::default()
                    })
                    .into(),
                ),
                material: materials.add(material.clone()),
                ..Default::default()
            })
            .insert(*element)
            .id();

        vector_ball_data
            .vectors
            .insert(*element, VectorParts { cylinder, cone });
    });
}

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum VectorBallElement {
    Momentum,
    Ball,
}

fn transform_vector_parts<'a>(
    scale: f32,
    vector: Vec3,
    origin: Vec3,
    cone: &'a mut Transform,
    cylinder: &'a mut Transform,
) {
    let rotation = Quat::from_rotation_arc(Vec3::Y, vector.normalize());
    let direction = vector.normalize();
    let length = vector.length();
    let unscaled_cylinder_length =
        (vector.length() - BALL_RADIUS - FLOAT_HEIGHT - CONE_HEIGHT).max(0.0);
    let unscaled_cylinder_translation = unscaled_cylinder_length / 2.0 + BALL_RADIUS + FLOAT_HEIGHT;

    cone.scale = Vec3::splat(scale);
    cone.rotation = rotation;
    cone.translation = direction * (length - CONE_HEIGHT / 2.0) * scale + origin;

    cylinder.scale = Vec3::new(scale, scale * unscaled_cylinder_length, scale);
    cylinder.rotation = rotation;
    cylinder.translation = direction * unscaled_cylinder_translation * scale + origin;
}

pub fn update_vector_ball(
    mut vector_ball_updates: EventReader<VectorBallUpdate>,
    mut vector_parts: Query<(&mut Transform, &mut Visibility), With<VectorBallElement>>,
    vector_ball_data: Res<VectorBallData>,
) {
    if vector_ball_updates.is_empty() {
        vector_parts.for_each_mut(|(_, mut visibility)| visibility.is_visible = false);
    }
    for VectorBallUpdate {
        element, // which vector are we talking about?
        vector,  // in which direction shall it point?
        origin,  // and where shall I put it?
    } in vector_ball_updates.iter()
    {
        let scale = vector_ball_data.scale;
        if let Some(ball) = vector_ball_data.ball {
            if let Ok((mut ball_transform, mut ball_visibility)) = vector_parts.get_mut(ball) {
                ball_transform.scale = Vec3::splat(scale);
                ball_transform.translation = *origin;
                ball_visibility.is_visible = true;
            } else {
                error!("{element:?} vector missing ball {ball:?}");
            }
        } else {
            error!("Vector ball not set");
        }
        if let Some(VectorParts { cone, cylinder }) = vector_ball_data.vectors.get(element) {
            if let Ok([cone, cylinder]) = vector_parts.get_many_mut([*cone, *cylinder]) {
                let (mut cone_transform, mut cone_visibility) = cone;
                let (mut cylinder_transform, mut cylinder_visibility) = cylinder;
                transform_vector_parts(
                    scale,
                    *vector,
                    *origin,
                    &mut cone_transform,
                    &mut cylinder_transform,
                );
                cone_visibility.is_visible = true;
                cylinder_visibility.is_visible = true;
            } else {
                error!("One of cone {cone:?} or cylinder {cylinder:?} missing");
            }
        } else {
            error!("Did not find vector parts for {element:?}");
        }
    }
}

const VB_SCALING_FACTOR: f32 = 1.0 / 30.0;

pub fn relay_vector_ball_updates(
    planet_query: Query<(&Transform, &Momentum)>,
    vector_ball_transform_query: Query<&GlobalTransform, With<VectorBallTransform>>,
    mut hot_planet_events: EventReader<HotPlanetEvent>,
    mut vector_ball_updates: EventWriter<VectorBallUpdate>,
) {
    for &HotPlanetEvent { planet, .. } in hot_planet_events.iter() {
        if let Ok((_, momentum)) = planet_query.get(planet) {
            let vector = momentum.velocity * momentum.mass * VB_SCALING_FACTOR;
            let origin = vector_ball_transform_query
                .get_single()
                .unwrap()
                .translation();
            vector_ball_updates.send(VectorBallUpdate {
                element: VectorBallElement::Momentum,
                vector,
                origin,
            });
        }
    }
}
