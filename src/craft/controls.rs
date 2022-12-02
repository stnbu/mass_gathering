use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::{EventReader, Local, Quat, Query, Res, Time, Transform, Vec3, With};

use super::Spacecraft;

#[derive(Default)]
pub struct AngularVelocity(Vec3);

pub fn control(
    mut spacecraft_query: Query<&mut Transform, With<Spacecraft>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut angular_velocity: Local<AngularVelocity>,
    time: Res<Time>,
) {
    let mouse_motion_scaling = 0.0005;
    for event in mouse_motion_events.iter() {
        angular_velocity.0.x += event.delta.y * mouse_motion_scaling;
        angular_velocity.0.y += event.delta.x * mouse_motion_scaling;
    }
    let mouse_wheel_scaling = mouse_motion_scaling * 15.0;
    for event in mouse_wheel_events.iter() {
        angular_velocity.0.z += event.y * mouse_wheel_scaling;
    }

    let rotation = angular_velocity.0 * time.delta_seconds();

    let mut transform = spacecraft_query.get_single_mut().unwrap();
    let local_x = transform.local_x();
    let local_y = transform.local_y();
    let local_z = transform.local_z();
    transform.rotate(Quat::from_axis_angle(local_x, rotation.x));
    transform.rotate(Quat::from_axis_angle(local_y, rotation.y));
    transform.rotate(Quat::from_axis_angle(local_z, rotation.z));
}
