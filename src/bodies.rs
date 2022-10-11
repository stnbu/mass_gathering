// https://github.com/Canleskis/bevy-particular-demo/blob/2b4cc3a063c075e11e06a90aa5ac9ad9edd270fc/src/nbody.rs

use bevy::{math::Vec3Swizzles, prelude::*};
use heron::{should_run, Acceleration, Velocity};
use particular::prelude::*;

//use bevy::math::Vec2 as Vect;

const G: f32 = 1.0;

#[derive(Particle)]
pub struct Body {
    position: Vec3,
    mu: f32,
    entity: Entity,
}

impl Body {
    pub fn new(position: Vec3, mu: f32, entity: Entity) -> Self {
        Self {
            position,
            mu,
            entity,
        }
    }
}

#[derive(Component)]
pub struct PointMass(f32);

pub struct ParticularPlugin;

impl Plugin for ParticularPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ParticleSet::<Body>::new())
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_run_criteria(should_run)
                    .with_system(sync_particle_set),
            )
            .add_system_set_to_stage(
                CoreStage::Update,
                SystemSet::new()
                    .with_run_criteria(should_run)
                    .with_system(accelerate_particles),
            );
    }
}

fn sync_particle_set(
    mut particle_set: ResMut<ParticleSet<Body>>,
    query: Query<(Entity, &GlobalTransform, &PointMass)>,
) {
    *particle_set = ParticleSet::new();
    query.for_each(|(entity, tranform, mass)| {
        let position = tranform.translation().xyz();
        particle_set.add_massive(Body::new(position, mass.0 * G, entity))
    })
}

fn accelerate_particles(
    mut particle_set: ResMut<ParticleSet<Body>>,
    mut query: Query<&mut Acceleration, With<PointMass>>,
) {
    for (body, gravity) in particle_set.result() {
        // if let Ok(mut acceleration) = query.get_mut(body.entity) {
        //     acceleration.linear = gravity.extend(0.0);
        // }
    }
}

#[derive(Bundle)]
struct BodyBundle {
    #[bundle]
    shape_bundle: PbrBundle,
    velocity: Velocity,
    acceleration: Acceleration,
    point_mass: PointMass,
}

use std::f32::consts::PI;

impl BodyBundle {
    pub fn new(
        position: Vec3,
        velocity: Velocity,
        mass: f32,
        point_mass: PointMass,
        color: Color,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        let radius = (mass / PI).sqrt();
        Self {
            shape_bundle: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius,
                    ..Default::default()
                })),
                material: materials.add(color.into()),
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            velocity,
            acceleration: Acceleration::default(),
            point_mass,
        }
    }
}
