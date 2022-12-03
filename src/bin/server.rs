use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use mass_gathering::{radius_to_mass, FullGame, Momentum, PhysicsConfig, PointMassBundle};

fn planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.5;
    for n in [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)] {
        for side in [1.0, -1.0] {
            let (a, b, c) = n;
            let (a, b, c) = (a * side, b * side, c * side);

            let position = Vec3::new(a * 6.0, b * 6.0, c * 6.0);
            let velocity = Vec3::ZERO; // position.cross(Vec3::Z).normalize() * 0.5 * 0.0;

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

    // cam
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(-Vec3::Z, Vec3::Y),
        ..Default::default()
    });
}

/*
use std::collections::HashMap;

struct PlanetConfig {
    mass: f32,
    color: Color,
}

struct GameConfig {
    planets: HashMap<u64, PlanetConfig>,
}

struct Locations(HashMap<u64, Vec3>);
 */

/*
commands.spawn(Camera3dBundle {
                transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(-Vec3::Z, Vec3::Y),
                ..Default::default()
            });
*/

fn main() {
    App::new()
        .insert_resource(PhysicsConfig { sims_per_frame: 2 })
        .add_plugins(FullGame)
        .add_startup_system(planets)
        .run();
}
