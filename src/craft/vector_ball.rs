use bevy::prelude::{
    Component, Entity, EventReader, EventWriter, Quat, Query, Res, Resource, Transform, Vec3,
    Visibility, With,
};

use super::*;
use bevy::log::error;

use crate::physics::Momentum;

const VB_SCALING_FACTOR: f32 = 1.0 / 30.0;

pub struct VectorBallUpdate {
    element: VectorBallElement,
    vector: Vec3,
    // Note that vector ball parts are children of spacecraft, therefore, this
    // is the "local" origin (where it sits in space as you look out the window).
    origin: Vec3,
}

pub struct VectorParts {
    pub cylinder: Entity,
    pub cone: Entity,
}
use std::collections::HashMap;

#[derive(Resource)]
pub struct VectorBallData {
    pub ball: Option<Entity>,
    pub container: Option<Entity>,
    pub vectors: HashMap<VectorBallElement, VectorParts>,
    pub scale: f32,
}

impl Default for VectorBallData {
    fn default() -> Self {
        Self {
            ball: None,
            container: None,
            vectors: HashMap::new(),
            scale: 0.03,
        }
    }
}

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum VectorBallElement {
    Momentum,
    Force,
    Ball,
    Container,
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

#[derive(Default)]
pub struct VectorBallPreviousValues(pub HashMap<VectorBallElement, Vec3>);

pub fn update_vector_ball(
    mut vector_ball_updates: EventReader<VectorBallUpdate>,
    mut vector_parts: Query<(&mut Transform, &mut Visibility), With<VectorBallElement>>,
    vector_ball_data: Res<VectorBallData>,
    mut prev_value: Local<VectorBallPreviousValues>,
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
        let longest = prev_value.0.values().fold(Vec3::ZERO, |longest, current| {
            if current.length() > longest.length() {
                *current
            } else {
                longest
            }
        });
        let longest_length = longest.length();
        let container_scale = vector_ball_data.scale;
        let scale = vector_ball_data.scale * 1.0 / longest_length;
        match *element {
            VectorBallElement::Container => {
                if let Some(container) = vector_ball_data.container {
                    if let Ok((mut container_transform, mut container_visibility)) =
                        vector_parts.get_mut(container)
                    {
                        container_transform.scale = Vec3::splat(container_scale);
                        container_transform.translation = *origin;
                        container_visibility.is_visible = true;
                    } else {
                        error!("{element:?} vector missing container {container:?}");
                    }
                } else {
                    error!("Vector container not set");
                }
            }
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
                        // Smoother code
                        let prev: Vec3 = if let Some(prev) = prev_value.0.get(element) {
                            *prev
                        } else {
                            *vector
                        };
                        let delta = *vector - prev;
                        let length = delta.length();
                        let coeff = if length >= 1.0 { 1.0 } else { length.powi(2) };
                        let vector = prev + delta * coeff;
                        prev_value.0.insert(*element, vector);
                        // End Smoother code

                        let (mut cone_transform, mut cone_visibility) = cone;
                        let (mut cylinder_transform, mut cylinder_visibility) = cylinder;
                        transform_vector_parts(
                            scale,
                            vector,
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
    mut hot_planet_events: EventReader<HotPlanetEvent>,
    mut vector_ball_updates: EventWriter<VectorBallUpdate>,
) {
    for &HotPlanetEvent { planet, .. } in hot_planet_events.iter() {
        if let Ok((_, momentum)) = planet_query.get(planet) {
            let origin = Vec3::new(-0.12, -0.06, -0.25);

            vector_ball_updates.send(VectorBallUpdate {
                element: VectorBallElement::Ball,
                vector: Vec3::ZERO,
                origin,
            });
            vector_ball_updates.send(VectorBallUpdate {
                element: VectorBallElement::Container,
                vector: Vec3::ZERO,
                origin,
            });
            let vector = momentum.velocity * momentum.mass * VB_SCALING_FACTOR;
            vector_ball_updates.send(VectorBallUpdate {
                element: VectorBallElement::Momentum,
                vector,
                origin,
            });

            let vector = momentum.force_ro * momentum.mass * VB_SCALING_FACTOR;
            vector_ball_updates.send(VectorBallUpdate {
                element: VectorBallElement::Force,
                vector,
                origin,
            });
        }
    }
}
