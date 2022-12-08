use crate::{
    server_connection_config, systems::spawn_planet, ClientChannel, ClientMessages, GameState,
    InitData, ServerChannel, ServerMessages, PORT_NUMBER, PROTOCOL_ID, SERVER_ADDR,
};
use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, RenetError};
use bevy_renet::renet::{RenetServer, ServerAuthentication, ServerConfig, ServerEvent};
use std::net::UdpSocket;
use std::time::SystemTime;

pub fn panic_on_renet_error(mut renet_error: EventReader<RenetError>) {
    for e in renet_error.iter() {
        panic!("{}", e);
    }
}

pub fn handle_client_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    mut client_messages: EventWriter<ClientMessages>,
    mut app_state: ResMut<State<GameState>>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::Init(init_data) => {
                info!(
                    "Server sent init data for {} planets to client {}",
                    init_data.planets.len(),
                    client.client_id()
                );
                info!("  spawning planets...");
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
                info!("  sending message to server `{message:?}`");
                client_messages.send(message);
            }
            ServerMessages::SetGameState(game_state) => {
                info!("Server says set state to {game_state:?}");
                let _ = app_state.overwrite_set(game_state);
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
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                info!("client {id} connected");
                let message = bincode::serialize(&ServerMessages::Init(init_data.clone())).unwrap();
                info!("sending initial data to client {id}");
                server.send_message(*id, ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected(id) => {
                info!("client {id} disconnected");
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
                    info!("Broadcasting {set_state:?}");
                    server.broadcast_message(ServerChannel::ServerMessages, message);
                    info!("  and setting my state to {state:?}");
                    let _ = app_state.overwrite_set(state);
                }
            }
        }
    }
}
