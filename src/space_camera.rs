use bevy::prelude::*;
use std::f32::consts::TAU;

struct SpaceCamera;

#[derive(Debug, Default)]
struct Curvature(Vec3);

impl Plugin for SpaceCamera {
    fn build(&self, app: &mut App) {
        app.insert_resource(Curvature::default())
            .add_startup_system(spawn_camera)
            .add_system(move_forward)
            .add_system(steer)
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
        ..Default::default()
    });
}

fn move_forward(mut camera_query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = camera_query.single_mut();
    let direction = transform.local_z();
    transform.translation -= direction * time.delta_seconds();
}

fn steer(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    mut curvature: ResMut<Curvature>,
) {
    let gain = 0.2;
    let nudge = TAU / 10000.0;
    let mut roll = 0.0;
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let mut had_input = false;
    for key in keys.get_pressed() {
        match key {
            KeyCode::Left => {
                roll += nudge * (curvature.0.z + 1.0);
                had_input = true;
                curvature.0.z += gain;
            }
            KeyCode::Right => {
                roll -= nudge * (curvature.0.z + 1.0);
                had_input = true;
                curvature.0.z += gain;
            }
            KeyCode::Up => {
                pitch -= nudge * (curvature.0.x + 1.0);
                had_input = true;
                curvature.0.x += gain;
            }
            KeyCode::Down => {
                pitch += nudge * (curvature.0.x + 1.0);
                had_input = true;
                curvature.0.x += gain;
            }
            KeyCode::Z => {
                yaw += nudge * (curvature.0.y + 1.0);
                had_input = true;
                curvature.0.y += gain;
            }
            KeyCode::X => {
                yaw -= nudge * (curvature.0.y + 1.0);
                had_input = true;
                curvature.0.y += gain;
            }
            _ => (),
        }
    }
    if !had_input {
        if curvature.0.x > 0.0 {
            curvature.0.x -= gain;
            if curvature.0.x < 0.0 {
                curvature.0.x = 0.0;
            }
        }
        if curvature.0.y > 0.0 {
            curvature.0.y -= gain;
            if curvature.0.y < 0.0 {
                curvature.0.y = 0.0;
            }
        }
        if curvature.0.z > 0.0 {
            curvature.0.z -= gain;
            if curvature.0.z < 0.0 {
                curvature.0.z = 0.0;
            }
        }
    }
    let mut transform = query.single_mut();
    if roll != 0.0 || pitch != 0.0 || yaw != 0.0 {
        let local_x = transform.local_x();
        let local_y = transform.local_y();
        let local_z = transform.local_z();
        transform.rotate(Quat::from_axis_angle(local_x, pitch));
        transform.rotate(Quat::from_axis_angle(local_z, roll));
        transform.rotate(Quat::from_axis_angle(local_y, yaw));
    }
}
