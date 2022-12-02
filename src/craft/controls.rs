use bevy::input::mouse::MouseMotion;
use bevy::prelude::{EventReader, Local, Quat, Query, Res, Time, Transform, Vec3, With};

use super::Spacecraft;

#[derive(Default)]
pub struct AngularVelocity(Vec3);

pub fn control(
    mut spacecraft_query: Query<&mut Transform, With<Spacecraft>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut angular_velocity: Local<AngularVelocity>,
    time: Res<Time>,
) {
    let mouse_scaling = 0.001;
    for event in mouse_motion_events.iter() {
        angular_velocity.0.x -= event.delta.y * mouse_scaling;
        angular_velocity.0.y -= event.delta.x * mouse_scaling;
    }

    let rotation = angular_velocity.0 * time.delta_seconds();

    let mut transform = spacecraft_query.get_single_mut().unwrap();

    let local_x = transform.local_x();
    let local_y = transform.local_y();
    transform.rotate(Quat::from_axis_angle(local_x, rotation.x));
    transform.rotate(Quat::from_axis_angle(local_y, rotation.y));
}
