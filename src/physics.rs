use bevy::prelude::*;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionEvent, GravityScale, RigidBody};
use std::collections::HashSet;
use std::f32::consts::PI;

pub fn collision_events(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    query: Query<(&Transform, &Momentum), With<Collider>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut despawned = HashSet::new();

    for collision_event in events.iter() {
        if let CollisionEvent::Started(e1, e2, flags) = collision_event {
            if despawned.contains(e1) || despawned.contains(e2) {
                continue;
            }
            if !flags.is_empty() {
                warn!("Unexpected collision flags: {:?}", flags);
            }
            if let Ok([planet0, planet1]) = query.get_many([*e1, *e2]) {
                let (planet0_transform, planet0_momentum) = planet0;
                let (planet1_transform, planet1_momentum) = planet1;
                let (weight0, weight1) = planet0_momentum.get_apportionment(planet1_momentum);
                let new_momentum = planet0_momentum.apportioned_new(planet1_momentum);
                let new_position = planet0_transform.translation * weight0
                    + planet1_transform.translation * weight1;
                let new_radius = mass_to_radius(new_momentum.mass);
                spawn_planet(
                    new_radius,
                    new_position,
                    new_momentum.velocity,
                    new_momentum.color,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );
            }
            commands.entity(*e1).despawn();
            commands.entity(*e2).despawn();
            despawned.insert(e1);
            despawned.insert(e2);
        }
    }
}

fn radius_to_mass(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * radius.powf(3.0)
}

fn mass_to_radius(mass: f32) -> f32 {
    ((mass * (3.0 / 4.0)) / PI).powf(1.0 / 3.0)
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
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius,
                ..Default::default()
            })),
            material: materials.add(color.into()),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .insert(Momentum {
            velocity,
            mass,
            color,
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(radius))
        .insert(ActiveEvents::COLLISION_EVENTS);
}

#[derive(Bundle)]
struct Planet {
    #[bundle]
    pbr: PbrBundle,
    point_mass: Transform,
}

#[derive(Component, Debug)]
pub struct Momentum {
    velocity: Vec3,
    mass: f32,
    color: Color,
}

impl Momentum {
    fn apportioned_new(&self, other: &Self) -> Self {
        let (self_weight, other_weight) = self.get_apportionment(other);
        let self_rgba = self
            .color
            .as_rgba_f32()
            .iter()
            .map(|f| *f * self_weight)
            .collect::<Vec<f32>>();
        let other_rgba = other
            .color
            .as_rgba_f32()
            .iter()
            .map(|f| *f * other_weight)
            .collect::<Vec<f32>>();
        let c = self_rgba
            .iter()
            .zip(other_rgba)
            .map(|(s, o)| s + o)
            .collect::<Vec<f32>>();
        Self {
            velocity: self_weight * self.velocity + other_weight * other.velocity,
            mass: self.mass + other.mass,
            color: Color::rgba(c[0], c[1], c[2], c[3]),
        }
    }

    fn get_apportionment(&self, other: &Self) -> (f32, f32) {
        (
            self.mass / (self.mass + other.mass),
            other.mass / (self.mass + other.mass),
        )
    }
}

impl Default for Momentum {
    fn default() -> Self {
        Momentum {
            velocity: Vec3::ZERO,
            mass: 0.0,
            color: Color::default(),
        }
    }
}

pub fn freefall(mut query: Query<(Entity, &mut Transform, &mut Momentum)>, time: Res<Time>) {
    let masses = query
        .iter()
        .map(|t| (t.0, t.1.translation, t.2.mass))
        .collect::<Vec<_>>();
    let accelerations = masses.iter().map(|particle1| {
        masses.iter().fold(Vec3::ZERO, |acceleration, particle2| {
            let dir = particle2.1 - particle1.1;
            let mag_2 = dir.length();
            let grav_acc = if mag_2 != 0.0 {
                dir * particle2.2 / (mag_2 * mag_2.sqrt())
            } else {
                dir
            };
            acceleration + grav_acc * 0.1
        })
    });
    let dt = time.delta_seconds();
    for ((entity, _, mass), force) in masses.iter().zip(accelerations) {
        if let Ok((_, mut transform, mut momentum)) = query.get_mut(*entity) {
            momentum.velocity += (force * dt) / *mass;
            transform.translation += momentum.velocity * dt;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::mass_to_radius;
    use super::radius_to_mass;
    use super::Momentum;

    #[test]
    fn gemoetry() {
        let mass = radius_to_mass(2.0);
        assert!(mass == 33.510323);
        let radius = mass_to_radius(33.510323);
        assert!(radius == 2.0);
    }

    #[test]
    fn apportionment() {
        let m0 = Momentum {
            mass: 69.0,
            ..Default::default()
        };
        let m1 = Momentum {
            mass: 42.0,
            ..Default::default()
        };
        let m01 = m0.apportioned_new(&m1);
        println!("{:?}", m01);
        assert!(m01.mass == 69.0 + 42.0);
        let (m0_weight, m1_weight) = m0.get_apportionment(&m1);
        assert!(m0_weight == 69.0 / (69.0 + 42.0));
        assert!(m1_weight == 42.0 / (69.0 + 42.0));
        let (m1_weight_flip, m0_weight_flip) = m1.get_apportionment(&m0);
        assert!(m0_weight == m0_weight_flip);
        assert!(m1_weight == m1_weight_flip);
    }
}
