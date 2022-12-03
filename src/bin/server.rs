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
            let id = commands
                .spawn(PointMassBundle {
                    pbr: PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Icosphere {
                            radius,
                            ..Default::default()
                        })),

                        material: materials.add(
                            Color::rgba((1.0 - a) / 2.0, (1.0 - b) / 2.0, (1.0 - c) / 2.0, 0.4)
                                .into(),
                        ),
                        transform: Transform::from_xyz(a * 6.0, b * 6.0, c * 6.0),
                        ..Default::default()
                    },
                    momentum: Momentum {
                        velocity: Vec3::ZERO,
                        mass: radius_to_mass(radius),
                        ..Default::default()
                    },
                    collider: Collider::ball(radius),
                    ..Default::default()
                })
                .id();
            debug!("Spawned (non-'spacecraft') planet {id:?}");
        }
    }
}

use std::collections::HashMap;

struct PlanetConfig {
    mass: f32,
    color: Color,
}

struct SpacecraftConfig_ {
    client_id: u64,
    mass: f32,
    color: Color,
}

struct GameConfig {
    planets: HashMap<u64, PlanetConfig>,
    spacecraft: HashMap<u64, SpacecraftConfig_>,
}

struct Locations(HashMap<u64, Vec3>);

fn main() {
    App::new()
        .insert_resource(PhysicsConfig { sims_per_frame: 2 })
        .add_plugins(FullGame)
        .add_startup_system(planets)
        .run();
}
