use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::TAU;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn latlon_to_cartesian(lat: f32, lon: f32) -> Vec3 {
    let theta = (lat * 2.0 - 1.0).acos(); // latitude. -1 & 1 are poles. 0 is equator.
    let phi = lon * TAU; // portion around the planet `[0,1)` (from Greenwich)
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();
    Vec3::new(x, y, z)
}

#[derive(Component)]
struct Spacecraft;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mut rf = || rng.gen::<f32>();
    let pair_count = 40;
    for _ in 0..pair_count {
        let position = latlon_to_cartesian(rf(), rf()) * (rf() * 40.0 + 10.0);
        let radius = rf() + 2.0;
        for side in [-1.0, 1.0] {
            let color = Color::rgb(rf(), rf(), rf());
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius,
                    ..Default::default()
                })),
                material: materials.add(color.into()),
                transform: Transform::from_translation(position * side),
                ..Default::default()
            });
        }
    }
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 75.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            ..Default::default()
        })
        .insert_bundle(VisibilityBundle::default())
        .insert(Spacecraft)
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 1.0,
                    ..Default::default()
                })),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_xyz(0.0, 0.0, -8.0),
                visibility: Visibility::visible(),
                ..Default::default()
            });
            parent.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(10.0, -10.0, -25.0),
                visibility: Visibility::visible(),
                point_light: PointLight {
                    intensity: 5000.0 * 1.7,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(-10.0, 5.0, -35.0),
                visibility: Visibility::visible(),
                point_light: PointLight {
                    intensity: 5000.0 * 1.5,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(30.0, -20.0, 80.0),
                visibility: Visibility::visible(),
                point_light: PointLight {
                    intensity: 1000000.0 * 0.7,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(-30.0, 10.0, 100.0),
                visibility: Visibility::visible(),
                point_light: PointLight {
                    intensity: 1000000.0 * 0.8,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
