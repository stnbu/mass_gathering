use crate::networking::ClientMessages;
use bevy::{
    math::EulerRot,
    prelude::{
        debug, Component, EventReader, EventWriter, Input, KeyCode, Quat, Query, Res, Time,
        Transform, Vec3, With,
    },
};
use std::f32::consts::TAU;

#[derive(Component)]
pub struct ClientInhabited;

#[derive(Component)]
pub struct Inhabitable;

pub struct ClientRotation(pub Quat);

pub fn rotate_client_inhabited_mass(
    mut inhabitant_query: Query<&mut Transform, With<ClientInhabited>>,
    mut rotation_events: EventReader<ClientRotation>,
) {
    let mut transform = inhabitant_query
        .get_single_mut()
        .expect("Could not get transform of client-inhabited entity");
    for ClientRotation(rotation) in rotation_events.iter() {
        transform.rotate(*rotation);
    }
}

pub fn control(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut rotation_events: EventWriter<ClientRotation>,
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
        rotation_events.send(ClientRotation(rotation));

        let message = ClientMessages::Rotation(rotation);
        debug!("  sending message to server `{message:?}`");
        client_messages.send(message);
    }
}
