use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseMotion, MouseWheel};
use bevy::input::ButtonState;
use bevy::prelude::{
    EventReader, EventWriter, Input, KeyCode, Quat, Query, Res, Time, Transform, Vec3,
};
use std::f32::consts::TAU;

use super::FireProjectileEvent;
use super::Spacecraft;

pub fn control(
    keys: Res<Input<KeyCode>>,
    mut spacecraft_query: Query<(&mut Transform, &mut Spacecraft)>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut fire_projectile_events: EventWriter<FireProjectileEvent>,
    time: Res<Time>,
) {
    // FIXME: this does not regard time delta, which it should.
    let nudge = TAU / 10000.0;

    let keys_scaling = 10.0;
    let mouse_scaling = 0.0001;

    // rotation about local axes
    let mut rotation = Vec3::ZERO;

    let (mut transform, mut spacecraft) = spacecraft_query
        .get_single_mut()
        .expect("Spacecraft query failed");

    // `just_presssed` ignores keys held down.
    for key in keys.get_just_pressed() {
        match key {
            KeyCode::PageUp => {
                spacecraft.speed += 1.0 + spacecraft.speed * 0.05;
            }
            KeyCode::PageDown => {
                spacecraft.speed -= 1.0 + spacecraft.speed * 0.05;
            }
            _ => {}
        }
    }

    // Make it easier to find "neutral"
    if spacecraft.speed.abs() < 0.5 {
        spacecraft.speed = 0.0
    }

    // `presssed` (contrast `just_pressed`) considers keys being _held_ down, which is good for rotation controls.
    for key in keys.get_pressed() {
        match key {
            KeyCode::A => {
                rotation.y += nudge;
            }
            KeyCode::D => {
                rotation.y -= nudge;
            }
            KeyCode::W => {
                rotation.x += nudge;
            }
            KeyCode::S => {
                rotation.x -= nudge;
            }
            KeyCode::Z => {
                rotation.z += nudge;
            }
            KeyCode::X => {
                rotation.z -= nudge;
            }
            KeyCode::Space => {
                fire_projectile_events.send(FireProjectileEvent);
            }
            _ => (),
        }
    }

    for event in mouse_motion_events.read() {
        rotation.x -= event.delta.y * mouse_scaling;
        rotation.y -= event.delta.x * mouse_scaling;
    }
    for MouseButtonInput { button, state, .. } in mouse_button_input_events.read() {
        if button == &MouseButton::Left && state == &ButtonState::Pressed {
            fire_projectile_events.send(FireProjectileEvent);
        }
    }
    for event in mouse_wheel_events.read() {
        spacecraft.speed = event.y;
    }

    let frame_time = time.delta_seconds() * 60.0;
    rotation *= keys_scaling * frame_time;

    let local_x = transform.local_x();
    let local_y = transform.local_y();
    let local_z = transform.local_z();
    transform.rotate(Quat::from_axis_angle(local_x, rotation.x));
    transform.rotate(Quat::from_axis_angle(local_z, rotation.z));
    transform.rotate(Quat::from_axis_angle(local_y, rotation.y));
}
