use bevy::prelude::*;

mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        //.add_system(camera::pan_orbit_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    for n in 0..10 {
        let mut side = 1.0;
        if n % 2 == 0 {
            side = -1.0;
        }
        let step = 2.0 * n as f32;
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(2.0 * side, 0.5, step),
            ..Default::default()
        });
    }
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.5, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }); //    let translation = Vec3::new(-2.0, 2.5, 5.0);
        //camera::spawn_camera(commands);
}
