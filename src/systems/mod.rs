use crate::{radius_to_mass, Momentum, PointMassBundle};
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

/// Make some interesting planets and set up a camera to watch
pub fn cubic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.5;
    for n in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
        for side in [1, -1] {
            let (a, b, c) = n;
            let bzz = 0.1;
            let velocity = match (a, b, c) {
                (1, 0, 0) => Vec3::Y * side as f32 * bzz,
                (0, 1, 0) => Vec3::Z * side as f32 * bzz,
                (0, 0, 1) => Vec3::X * side as f32 * bzz,
                _ => panic!(),
            };
            let (a, b, c) = (a * side, b * side, c * side);
            let (a, b, c) = (a as f32, b as f32, c as f32);

            let position = Vec3::new(a * 6.0, b * 6.0, c * 6.0);

            commands.spawn(PointMassBundle {
                pbr: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius,
                        ..Default::default()
                    })),
                    material: materials.add(
                        Color::rgba((1.0 - a) / 2.0, (1.0 - b) / 2.0, (1.0 - c) / 2.0, 0.4).into(),
                    ),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
                momentum: Momentum {
                    velocity,
                    mass: radius_to_mass(radius),
                    ..Default::default()
                },
                collider: Collider::ball(radius),
                ..Default::default()
            });
        }
    }
}
