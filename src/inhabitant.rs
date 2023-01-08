use crate::networking::ClientMessages;
use bevy::{
    math::EulerRot,
    prelude::{
        debug, Component, EventReader, EventWriter, Input, KeyCode, Quat, Query, Res, ResMut, Time,
        Transform, Vec3, With,
    },
};
use std::f32::consts::TAU;

// Note that "client inhabited" means "me", as in, the mass inhabited
// by _this_ client, the one that has your camera attached to it.

#[derive(Component)]
pub struct ClientInhabited;

#[derive(Component)]
pub struct Inhabitable;

pub fn control(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut client_messages: EventWriter<ClientMessages>,
) {
    let nudge = TAU / 10000.0;
    let keys_scaling = 10.0;

    // rotation about local axes
    let mut rotation = Vec3::ZERO;

    // IDEAR: we could just get key counts as f32 and multiply by nudge.
    //   A -> [0, 0, 1]
    //   D -> [0, 0, -1]
    // ...etc
    for key in keys.get_pressed() {
        match key {
            KeyCode::A => {
                rotation.y += nudge;
            }
            KeyCode::D => {
                rotation.y -= nudge;
            }
            KeyCode::W => {
                rotation.z -= nudge;
            }
            KeyCode::S => {
                rotation.z += nudge;
            }
            KeyCode::Z => {
                rotation.x += nudge;
            }
            KeyCode::X => {
                rotation.x -= nudge;
            }
            _ => (),
        }
    }

    if rotation.length() > 0.0000001 {
        let frame_time = time.delta_seconds() * 60.0;
        let [x, y, z] = (rotation * keys_scaling * frame_time).to_array();
        let rotation = Quat::from_euler(EulerRot::XYZ, x, y, z);

        let message = ClientMessages::Rotation(rotation);
        client_messages.send(message);
    }
}

// Rotate ME by reading local Rotation events, independant of client/server.
pub fn rotate_client_inhabited_mass(
    mut client_messages: EventReader<ClientMessages>,
    mut inhabitant_query: Query<&mut Transform, With<ClientInhabited>>,
) {
    if let Ok(mut transform) = inhabitant_query.get_single_mut() {
        for message in client_messages.iter() {
            if let ClientMessages::Rotation(rotation) = message {
                transform.rotate(*rotation);
            }
        }
    } else {
        debug!("ClientInhabited entity not present");
    }
}
