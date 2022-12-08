use bevy::prelude::*;
use bevy_renet::renet::RenetConnectionConfig;
use bevy_renet::renet::{
    DefaultChannel, RenetServer, ServerAuthentication, ServerConfig, ServerEvent,
};
use std::{net::UdpSocket, time::SystemTime};

use crate::networking::*;
use crate::GameState;

pub fn new_renet_server() -> RenetServer {
    let server_addr = format!("{SERVER_ADDR}:{PORT_NUMBER}").parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = server_connection_config();
    let server_config =
        ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

pub fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    init_data: Res<InitData>,
    mut app_state: ResMut<State<GameState>>,
    mut lobby: ResMut<Lobby>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, user_data) => {
                let id = *id;
                debug!("Server got connect from client {id}");
                let message = bincode::serialize(&ServerMessages::Init(init_data.clone())).unwrap();
                debug!("Server sending initial data to client {id}");
                server.send_message(id, ServerChannel::ServerMessages, message.clone());
                let client_preferences = ClientPreferences::from_user_data(user_data);
                lobby.clients.insert(id, client_preferences);
                let client_preferences = client_preferences.clone();
                let message = ServerMessages::ClientConnected {
                    id,
                    client_preferences,
                };
                for &id in lobby.clients.keys() {
                    debug!("  sending {message:?} to client {id}");
                    server.send_message(
                        id,
                        DefaultChannel::Reliable,
                        bincode::serialize(&message).unwrap(),
                    );
                }
                debug!("  broadcasting {message:?}");
                server.broadcast_message(
                    DefaultChannel::Reliable,
                    bincode::serialize(&message).unwrap(),
                );
            }
            ServerEvent::ClientDisconnected(id) => {
                debug!("Server got disconnect from client {id}");
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::ClientMessages) {
            match message {
                _ => {
                    // FIXME: we need to do this "when everybody is ready"
                    let state = GameState::Running;
                    let set_state = ServerMessages::SetGameState(state);
                    let message = bincode::serialize(&set_state).unwrap();
                    debug!("Broadcasting {set_state:?}");
                    server.broadcast_message(ServerChannel::ServerMessages, message);
                    debug!("  and setting my state to {state:?}");
                    let _ = app_state.overwrite_set(state);
                }
            }
        }
    }
}

pub fn server_connection_config() -> RenetConnectionConfig {
    RenetConnectionConfig {
        send_channels_config: ServerChannel::channels_config(),
        ..Default::default()
    }
}
