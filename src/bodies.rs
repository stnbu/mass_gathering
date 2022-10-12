use bevy::prelude::*;
use particular::prelude::*;

#[derive(Particle)]
pub struct Body {
    position: Vec3,
    mu: f32,
    velocity: Vec3,
    entity: Entity,
}

impl Body {
    pub fn new(position: Vec3, mu: f32, velocity: Vec3, entity: Entity) -> Self {
        Self {
            position,
            mu,
            velocity,
            entity,
        }
    }
}

#[derive(Component)]
pub struct PointMass;

pub fn update_particles(
    mut particle_set: ResMut<ParticleSet<Body>>,
    mut query: Query<&mut Transform, With<PointMass>>,
    time: Res<Time>,
) {
    let d = time.delta_seconds();
    for (particle, force) in particle_set.result() {
        particle.velocity += (force * d) / particle.mu;
        particle.position += particle.velocity * d;
        if let Ok(mut transform) = query.get_mut(particle.entity) {
            transform.translation = particle.position;
        }
    }
}
