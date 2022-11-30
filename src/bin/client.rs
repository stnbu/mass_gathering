use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use mass_gathering::{
    radius_to_mass, FullGame, Momentum, PhysicsConfig, PointMassBundle, SpacecraftConfig,
};

fn planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for n in [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)] {
        let (a, b, c) = n;
        commands.spawn(PointMassBundle {
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 1.0,
                    ..Default::default()
                })),
                material: materials.add(Color::rgb(a, b, c).into()),
                transform: Transform::from_xyz(a * 50.0, b * 50.0, c * 50.0),
                ..Default::default()
            },
            momentum: Momentum {
                velocity: Vec3::ZERO,
                mass: radius_to_mass(10.0),
                ..Default::default()
            },
            collider: Collider::ball(1.0),
            ..Default::default()
        });
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(PhysicsConfig {
            sims_per_frame: 1,
            trails: true,
            trail_ttl: 2500 * 5,
        })
        .insert_resource(SpacecraftConfig {
            start_transform: Transform::from_xyz(0.0, 0.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
            impact_magnitude: 5.0,
            ..Default::default()
        })
        .add_plugins(FullGame)
        .add_startup_system(planets)
        .run();
}
