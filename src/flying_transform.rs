use bevy::prelude::*;
use std::f32::consts::TAU;

pub type FlyingTransform = Transform;

#[derive(Debug, Default, Component)]
pub struct Movement {
    gain: Vec3,
    pub speed: f32,
}

pub fn move_forward(mut query: Query<(&mut FlyingTransform, &Movement)>, time: Res<Time>) {
    for (mut transform, movement) in query.iter_mut() {
        let direction = transform.local_z();
        transform.translation -= direction * time.delta_seconds() * movement.speed;
    }
}

pub fn steer(keys: Res<Input<KeyCode>>, mut query: Query<(&mut FlyingTransform, &mut Movement)>) {
    let gain = 0.2;
    let nudge = TAU / 10000.0;
    let mut roll = 0.0;
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let mut had_input = false;

    let (mut transform, mut movement) = query.get_single_mut().unwrap();

    for key in keys.get_pressed() {
        had_input = true;
        match key {
            KeyCode::Left => {
                roll += nudge * (movement.gain.z + 1.0);
                movement.gain.z += gain;
            }
            KeyCode::Right => {
                roll -= nudge * (movement.gain.z + 1.0);
                movement.gain.z += gain;
            }
            KeyCode::Up => {
                pitch += nudge * (movement.gain.x + 1.0);
                movement.gain.x += gain;
            }
            KeyCode::Down => {
                pitch -= nudge * (movement.gain.x + 1.0);
                movement.gain.x += gain;
            }
            KeyCode::Z => {
                yaw += nudge * (movement.gain.y + 1.0);
                movement.gain.y += gain;
            }
            KeyCode::X => {
                yaw -= nudge * (movement.gain.y + 1.0);
                movement.gain.y += gain;
            }
            KeyCode::PageUp => {
                movement.speed += 0.5;
            }
            KeyCode::PageDown => {
                movement.speed -= 0.5;
            }
            _ => {
                had_input = false;
            }
        }
    }

    if !had_input {
        if movement.gain.x > 0.0 {
            movement.gain.x -= gain;
            if movement.gain.x < 0.0 {
                movement.gain.x = 0.0;
            }
        }
        if movement.gain.y > 0.0 {
            movement.gain.y -= gain;
            if movement.gain.y < 0.0 {
                movement.gain.y = 0.0;
            }
        }
        if movement.gain.z > 0.0 {
            movement.gain.z -= gain;
            if movement.gain.z < 0.0 {
                movement.gain.z = 0.0;
            }
        }
    }

    if roll != 0.0 || pitch != 0.0 || yaw != 0.0 {
        let local_x = transform.local_x();
        let local_y = transform.local_y();
        let local_z = transform.local_z();
        transform.rotate(Quat::from_axis_angle(local_x, pitch));
        transform.rotate(Quat::from_axis_angle(local_z, roll));
        transform.rotate(Quat::from_axis_angle(local_y, yaw));
    }
}

//pub type RelativeTransform = Transform;
#[derive(Component, Default)]
pub struct RelativeTransform(pub Transform);

pub fn update_relative_transforms(
    mut followers: Query<
        (&mut Transform, &RelativeTransform),
        (With<RelativeTransform>, Without<FlyingTransform>),
    >,
    flying_transform_query: Query<
        &FlyingTransform,
        (Without<RelativeTransform>, With<FlyingTransform>),
    >,
) {
    for (mut follower, relative_transform) in followers.iter_mut() {
        if let Ok(frame) = flying_transform_query.get_single() {
            *follower = frame.mul_transform((*relative_transform).0);
        }
    }
}
