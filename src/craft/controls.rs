use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::prelude::{EventReader, Input, KeyCode, Quat, Query, Res, Time, Transform, Vec2};
use std::f32::consts::TAU;

use super::Spacecraft;

pub fn keyboard_control(
    keys: Res<Input<KeyCode>>,
    mut spacecraft_query: Query<(&mut Transform, &mut Spacecraft)>,

    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    // FIXME: this does not regard time delta, which it should.
    let nudge = TAU / 10000.0;
    let mut roll = 0.0;
    let mut pitch = 0.0;
    let mut yaw = 0.0;

    let (mut transform, mut spacecraft) = spacecraft_query.get_single_mut().unwrap();

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
                yaw += nudge;
            }
            KeyCode::D => {
                yaw -= nudge;
            }
            KeyCode::W => {
                pitch += nudge;
            }
            KeyCode::S => {
                pitch -= nudge;
            }
            KeyCode::Z => {
                roll += nudge;
            }
            KeyCode::X => {
                roll -= nudge;
            }
            _ => (),
        }
    }

    // // // MOUSE
    let mut mouse_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        mouse_delta += event.delta;
    }
    if mouse_delta.length() > 0.0 {
        //mouse_delta.y * 0.25 * dt;
        //mouse_delta.x * 0.5 * dt
    }
    for event in mouse_button_input_events.iter() {}
    for event in mouse_wheel_events.iter() {}
    // // // END MAUS

    let local_x = transform.local_x();
    let local_y = transform.local_y();
    let local_z = transform.local_z();
    transform.rotate(Quat::from_axis_angle(local_x, pitch));
    transform.rotate(Quat::from_axis_angle(local_z, roll));
    transform.rotate(Quat::from_axis_angle(local_y, yaw));
}
