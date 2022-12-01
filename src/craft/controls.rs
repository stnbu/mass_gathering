use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseMotion};
use bevy::input::ButtonState;
use bevy::prelude::{EventReader, EventWriter, Quat, Query, Transform, Vec3, With};

use super::FireProjectileEvent;
use super::Spacecraft;

pub fn control(
    mut spacecraft_query: Query<&mut Transform, With<Spacecraft>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut fire_projectile_events: EventWriter<FireProjectileEvent>,
) {
    let mouse_scaling = 0.0001;

    // rotation about local axes
    let mut rotation = Vec3::ZERO;

    let mut transform = spacecraft_query.get_single_mut().unwrap();

    for event in mouse_motion_events.iter() {
        rotation.x -= event.delta.y * mouse_scaling;
        rotation.y -= event.delta.x * mouse_scaling;
    }
    for MouseButtonInput { button, state } in mouse_button_input_events.iter() {
        if *button == MouseButton::Left && *state == ButtonState::Pressed {
            fire_projectile_events.send(FireProjectileEvent);
        }
    }
    let local_x = transform.local_x();
    let local_y = transform.local_y();
    let local_z = transform.local_z();
    transform.rotate(Quat::from_axis_angle(local_x, rotation.x));
    transform.rotate(Quat::from_axis_angle(local_z, rotation.z));
    transform.rotate(Quat::from_axis_angle(local_y, rotation.y));
}
