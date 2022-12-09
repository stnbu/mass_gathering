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
                let new_id = *id;
                let client_preferences = ClientPreferences::from_user_data(user_data);
                debug!("Server got connection from new client {new_id} with preferences {client_preferences:?}");

                debug!("  sending initial data to client {new_id}");
                let message = bincode::serialize(&ServerMessages::Init(init_data.clone())).unwrap();
                server.send_message(new_id, ServerChannel::ServerMessages, message.clone());

                debug!("  replaying existing lobby back to new client {new_id:?}");
                for (&id, &client_preferences) in lobby.clients.iter() {
                    let message = ServerMessages::ClientConnected {
                        id,
                        client_preferences,
                    };
                    server.send_message(
                        new_id,
                        DefaultChannel::Reliable,
                        bincode::serialize(&message).unwrap(),
                    );
                }

                debug!("  now updating my lobby with client {new_id} preferences {client_preferences:?}");
                lobby.clients.insert(new_id, client_preferences);
                debug!("  the server now has lobby {lobby:?}");

                let message = ServerMessages::ClientConnected {
                    id: new_id,
                    client_preferences,
                };
                debug!("  broadcasting about new client: {message:?}");
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
            let message = bincode::deserialize(&message).unwrap();
            const SLOTS: usize = 3;
            debug!("Received message from client: {message:?}");
            match message {
                ClientMessages::Ready => {
                    let unanimous_autostart = lobby.clients.len() > 1
                        && lobby.clients.iter().all(|(_, prefs)| prefs.autostart);
                    if unanimous_autostart {
                        debug!("  two or more clients connected and all want to autostart.");
                    }
                    let game_full = lobby.clients.len() == SLOTS;
                    if game_full {
                        debug!("  game has now reached max capacity.");
                    }
                    let start = unanimous_autostart || game_full;
                    let state = if start {
                        GameState::Running
                    } else {
                        GameState::Waiting
                    };
                    let set_state = ServerMessages::SetGameState(state);
                    let message = bincode::serialize(&set_state).unwrap();
                    if start {
                        debug!("Broadcasting {set_state:?}");
                        server.broadcast_message(ServerChannel::ServerMessages, message);
                    } else {
                        debug!("Replying to client {client_id} with {set_state:?}");
                        server.send_message(client_id, DefaultChannel::Reliable, message);
                    }
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

pub fn set_window_title(game_state: Res<State<GameState>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_title(format!("Server[{:?}]", game_state.current()));
}
