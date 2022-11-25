use bevy::prelude::{
    shape, Assets, Color, Commands, Component, Entity, EventReader, EventWriter, GlobalTransform,
    Mesh, PbrBundle, Quat, Query, Res, ResMut, Resource, StandardMaterial, Transform, Vec3,
    Visibility, With,
};

use bevy::log::error;

use super::HotPlanetEvent;

use crate::physics::Momentum;

// //

use crate::mg_shapes::*;

const VB_SCALING_FACTOR: f32 = 1.0 / 30.0;

const BALL_RADIUS: f32 = 3.5 / 14.0;
const FLOAT_HEIGHT: f32 = 2.0 / 14.0;
const CYLINDER_RADIUS: f32 = 1.0 / 14.0;
const CONE_HEIGHT: f32 = 2.0 / 14.0;
const CONE_RADIUS: f32 = 2.0 / 14.0;

#[derive(Component)]
pub struct VectorBallTransform;

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

#[derive(Resource)]
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
    let crt_green_dark = StandardMaterial {
        base_color: Color::rgb(51.0, 255.0, 0.0),
        ..Default::default()
    };
    let crt_green_medium = StandardMaterial {
        base_color: Color::rgb(102.0, 255.0, 102.0),
        ..Default::default()
    };
    let crt_amber_light = StandardMaterial {
        base_color: Color::rgb(255.0, 176.0, 0.0),
        ..Default::default()
    };

    let ball = commands
        .spawn(PbrBundle {
            visibility: Visibility { is_visible: false },
            mesh: meshes.add(
                (shape::Icosphere {
                    radius: BALL_RADIUS,
                    ..Default::default()
                })
                .into(),
            ),
            material: materials.add(crt_green_dark.clone()),
            ..Default::default()
        })
        .insert(VectorBallElement::Ball)
        .id();
    vector_ball_data.ball = Some(ball);

    [VectorBallElement::Momentum].iter().for_each(|element| {
        let cone = commands
            .spawn(PbrBundle {
                visibility: Visibility { is_visible: false },
                mesh: meshes.add(
                    (Cone {
                        radius: CONE_RADIUS,
                        height: CONE_HEIGHT,
                        ..Default::default()
                    })
                    .into(),
                ),
                material: materials.add(crt_green_medium.clone()),
                ..Default::default()
            })
            .insert(*element)
            .id();
        let cylinder = commands
            .spawn(PbrBundle {
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
                material: materials.add(crt_amber_light.clone()),
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
    Force,
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
        match *element {
            VectorBallElement::Ball => {
                if let Some(ball) = vector_ball_data.ball {
                    if let Ok((mut ball_transform, mut ball_visibility)) =
                        vector_parts.get_mut(ball)
                    {
                        ball_transform.scale = Vec3::splat(scale);
                        ball_transform.translation = *origin;
                        ball_visibility.is_visible = true;
                    } else {
                        error!("{element:?} vector missing ball {ball:?}");
                    }
                } else {
                    error!("Vector ball not set");
                }
            }
            _ => {
                if let Some(VectorParts { cone, cylinder }) = vector_ball_data.vectors.get(element)
                {
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
    }
}

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
