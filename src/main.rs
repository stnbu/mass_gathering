use bevy::prelude::*;
use std::f32::consts::TAU;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_camera)
        .add_system(steer)
        .run();
}

fn move_camera(mut camera_query: Query<&mut Transform, With<Camera>>, timer: Res<Time>) {
    let mut transform = camera_query.single_mut();
    let direction = transform.local_z();
    transform.translation -= direction * timer.delta_seconds();
}

fn steer(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Camera>>) {
    let nudge = TAU / 1000.0;
    let mut right = 0.0;
    let mut up = 0.0;
    for key in keys.get_pressed() {
        match key {
            KeyCode::Left => right -= nudge,
            KeyCode::Right => right += nudge,
            KeyCode::Up => up += nudge,
            KeyCode::Down => up -= nudge,
            _ => (),
        }
    }
    if right != 0.0 || up != 0.0 {
        let mut transform = query.single_mut();
        let local_x = transform.local_x();
        let local_z = transform.local_z();
        transform.rotate(Quat::from_axis_angle(local_x, up));
        transform.rotate(Quat::from_axis_angle(local_z, right));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 60.0 })),
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
        transform: Transform::from_xyz(0.0, 0.5, -1.0)
            .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        ..Default::default()
    });
}
