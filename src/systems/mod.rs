use crate::{radius_to_mass, InitData, Momentum, PlanetInitData, PointMassBundle};
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

/// Make some interesting planets
pub fn cubic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut planet_data: ResMut<InitData>,
) {
    let mut planet_id = 2000;
    let radius = 0.5;
    let from_origin = 6.0;
    for n in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
        for side in [1.0, -1.0] {
            let (a, b, c) = n;
            let speed = 0.1;
            let position = Vec3::new(
                a as f32 * side * from_origin,
                b as f32 * side * from_origin,
                c as f32 * side * from_origin,
            );
            let velocity = match (a, b, c) {
                (1, 0, 0) => Vec3::Y * side,
                (0, 1, 0) => Vec3::Z * side,
                (0, 0, 1) => Vec3::X * side,
                _ => panic!(),
            } * speed;
            let color = Color::rgba(
                (1.0 - a as f32) / 2.0,
                (1.0 - b as f32) / 2.0,
                (1.0 - c as f32) / 2.0,
                0.8,
            );
            let planet_init_data = PlanetInitData {
                position,
                velocity,
                color,
                radius,
            };
            planet_data.planets.insert(planet_id, planet_init_data);
            planet_id += 1; // !
            commands.spawn(PointMassBundle {
                pbr: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius,
                        ..Default::default()
                    })),
                    material: materials.add(color.into()),
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
