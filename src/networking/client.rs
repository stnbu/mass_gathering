use crate::{systems::spawn_planet, GameConfig, GameState};
use bevy::prelude::*;
use bevy_renet::renet::{ClientAuthentication, RenetClient, RenetConnectionConfig};
use std::{net::UdpSocket, time::SystemTime};

use crate::networking::*;

pub fn handle_client_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    mut client_messages: EventWriter<ClientMessages>,
    mut app_state: ResMut<State<GameState>>,
    mut lobby: ResMut<Lobby>,
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
                    spawn_planet(
                        planet_id,
                        planet_init_data,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                    );
                }
                let message = ClientMessages::Ready;
                debug!("  sending message to server `{message:?}`");
                client_messages.send(message);
            }
            ServerMessages::SetGameState(game_state) => {
                debug!("Server says set state to {game_state:?}. Setting state now.");
                let _ = app_state.overwrite_set(game_state);
            }
            ServerMessages::ClientConnected {
                id,
                client_preferences,
            } => {
                debug!(
                    "Server says ({}, {:?}) connected. Updating my lobby.",
                    id, client_preferences
                );
                if let Some(old) = lobby.clients.insert(id, client_preferences) {
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
