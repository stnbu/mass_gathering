use crate::{systems::spawn_planet, GameConfig, GameState};
use bevy::input::{
    mouse::{MouseButton, MouseButtonInput, MouseMotion, MouseWheel},
    ButtonState,
};
use bevy::prelude::*;
use bevy_renet::renet::{ClientAuthentication, RenetClient, RenetConnectionConfig};
use std::f32::consts::TAU;
use std::{net::UdpSocket, time::SystemTime};

use crate::networking::*;

#[derive(Component)]
pub struct Inhabited;

pub fn handle_client_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    mut client_messages: EventWriter<ClientMessages>,
    mut game_state: ResMut<State<GameState>>,
    mut mass_to_entity_map: ResMut<MapMassIDToEntity>,
    mut lobby: ResMut<Lobby>, // maybe "lobby" should store init_data
    camera: Query<Entity, With<Camera>>, // FIXME. finer tuning.
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::Init(init_data) => {
                debug!(
                    "Server sent init data for {} planets to me, client {}",
                    init_data.planets.len(),
                    client.client_id()
                );
                debug!("  spawning planets...");
                for (&planet_id, &planet_init_data) in init_data.planets.iter() {
                    let entity_id = spawn_planet(
                        planet_id,
                        planet_init_data,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                    );
                    mass_to_entity_map.0.insert(planet_id, entity_id);
                }
                let message = ClientMessages::Ready;
                debug!("  sending message to server `{message:?}`");
                client_messages.send(message);
            }
            ServerMessages::SetGameState(new_game_state) => {
                // FIXME: Why in the _whorld_ would we receive this? --
                // "Server says set state to ResMut(
                //     State { transition: None, stack: [Stopped], scheduled: None, end_next_loop: false }
                // ). Setting state now."
                debug!("Server says set state to {game_state:?}. Setting state now.");
                let _ = game_state.overwrite_set(new_game_state);
            }
            ServerMessages::ClientJoined { id, client_data } => {
                debug!(
                    "Server says ({}, {:?}) connected. Updating my lobby.",
                    id, client_data
                );
                if id == client.client_id() {
                    debug!("  fyi, that's me (I am {id})");
                    let camera_id = camera.get_single().expect("Not exaclty one camera?");
                    debug!("  found exactly one existing camera: {camera_id:?}");
                    let inhabited_mass = mass_to_entity_map
                        .0
                        .get(&client_data.inhabited_mass_id)
                        .unwrap();
                    debug!("  found exactly one mass for me to inhabit: {inhabited_mass:?}");
                    debug!("  making {camera_id:?} a child of {inhabited_mass:?}");
                    commands
                        .entity(*inhabited_mass)
                        .insert(Inhabited)
                        .add_child(camera_id);
                }
                if let Some(old) = lobby.clients.insert(id, client_data) {
                    debug!("  the value {old:?} was replaced for client {id}");
                }
                debug!("  client {} now has lobby {lobby:?}", client.client_id());
            }
        }
    }
}

pub fn send_client_messages(
    mut client_messages: EventReader<ClientMessages>,
    mut client: ResMut<RenetClient>,
) {
    for command in client_messages.iter() {
        let message = bincode::serialize(command).unwrap();
        client.send_message(ClientChannel::ClientMessages, message);
    }
}

pub fn client_connection_config() -> RenetConnectionConfig {
    RenetConnectionConfig {
        receive_channels_config: ServerChannel::channels_config(),
        ..Default::default()
    }
}

pub fn new_renet_client(client_id: u64, client_preferences: ClientPreferences) -> RenetClient {
    let server_addr = format!("{SERVER_ADDR}:{PORT_NUMBER}").parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let connection_config = client_connection_config();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: Some(client_preferences.to_netcode_user_data()),
    };
    RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
}

pub fn set_window_title(
    game_state: Res<State<GameState>>,
    mut windows: ResMut<Windows>,
    game_config: Res<GameConfig>,
) {
    let nick = if game_config.nick.is_empty() {
        "<unset>"
    } else {
        &game_config.nick
    };
    let window = windows.primary_mut();
    window.set_title(format!("Client[{:?}] : nick={nick}", game_state.current()));
}

pub fn control(
    keys: Res<Input<KeyCode>>,
    mut inhabitant_query: Query<&mut Transform, With<Inhabited>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    let mut transform = if let Ok(transform) = inhabitant_query.get_single_mut() {
        transform
    } else {
        error!("Inhabitant missing!");
        return;
    };

    let nudge = TAU / 10000.0;
    let keys_scaling = 10.0;
    let mouse_scaling = 0.0001;

    // rotation about local axes
    let mut rotation = Vec3::ZERO;

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
            _ => (),
        }
    }

    for event in mouse_motion_events.iter() {
        rotation.x -= event.delta.y * mouse_scaling;
        rotation.y -= event.delta.x * mouse_scaling;
    }
    for MouseButtonInput { button, state } in mouse_button_input_events.iter() {
        if *state == ButtonState::Pressed {
            if *button == MouseButton::Left {
                println!("LEFT CLICK!");
            }
            if *button == MouseButton::Right {
                println!("RIGHT CLICK!");
            }
        }
    }
    let mouse_wheel_scaling = mouse_scaling * 15.0;
    for event in mouse_wheel_events.iter() {
        rotation.z += event.y * mouse_wheel_scaling;
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
