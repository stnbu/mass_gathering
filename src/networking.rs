use bevy_renet::renet::ClientAuthentication;
use bevy_renet::renet::{
    ChannelConfig, ReliableChannelConfig, RenetClient, RenetConnectionConfig, NETCODE_KEY_BYTES,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::time::Duration;
use std::{net::UdpSocket, time::SystemTime};

use crate::{systems::spawn_planet, GameState};
use bevy::prelude::*;
use bevy_renet::renet::RenetError;
use bevy_renet::renet::{RenetServer, ServerAuthentication, ServerConfig, ServerEvent};

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct PlanetInitData {
    pub position: Vec3,
    pub velocity: Vec3,
    pub color: Color,
    pub radius: f32,
}

#[derive(Default, Serialize, Deserialize, Resource, Debug)]
pub struct InitData {
    pub planets: HashMap<u64, PlanetInitData>,
}

impl Clone for InitData {
    fn clone(&self) -> Self {
        let mut planets = HashMap::new();
        planets.extend(&self.planets);
        Self { planets }
    }

    fn clone_from(&mut self, source: &Self) {
        let mut planets = HashMap::new();
        planets.extend(&source.planets);
        self.planets = planets;
    }
}

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

pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"dwxe_SERxx29e0)cs2@66#vxo0s5np{_";
pub const PROTOCOL_ID: u64 = 17;
pub const SERVER_ADDR: &str = "127.0.0.1";
pub const PORT_NUMBER: u16 = 5247;

pub enum ServerChannel {
    ServerMessages,
}

#[derive(Serialize, Deserialize, Component, Debug)]
pub enum ServerMessages {
    Init(InitData),
    SetGameState(GameState),
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::ServerMessages => 0,
        }
    }
}

impl ServerChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![ReliableChannelConfig {
            channel_id: Self::ServerMessages.into(),
            message_resend_time: Duration::from_millis(200),
            ..Default::default()
        }
        .into()]
    }
}

pub fn client_connection_config() -> RenetConnectionConfig {
    RenetConnectionConfig {
        receive_channels_config: ServerChannel::channels_config(),
        ..Default::default()
    }
}

pub fn server_connection_config() -> RenetConnectionConfig {
    RenetConnectionConfig {
        send_channels_config: ServerChannel::channels_config(),
        ..Default::default()
    }
}

#[derive(Component)]
pub struct MassID(pub u64);

pub fn spawn_arena_view_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(20.0, 18.0, 23.0).looking_at(-Vec3::Z, Vec3::Y),
        ..Default::default()
    });
}

//

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessages {
    Ready,
}

pub enum ClientChannel {
    ClientMessages,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::ClientMessages => 0,
        }
    }
}

impl ClientChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![ReliableChannelConfig {
            channel_id: Self::ClientMessages.into(),
            message_resend_time: Duration::ZERO,
            ..Default::default()
        }
        .into()]
    }
}

pub fn new_renet_client(client_id: u64) -> RenetClient {
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
        user_data: None,
    };
    RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
}

pub fn to_nick(id: u64) -> String {
    let nic_vec: Vec<u8> = id.to_ne_bytes().to_vec();
    String::from_utf8(nic_vec).unwrap().trim_end().to_string()
}

pub fn from_nick(nick: &str) -> u64 {
    let mut nick_vec = [b' '; 8];
    if nick.len() > 8 {
        panic!()
    }
    for (i, c) in nick.as_bytes().iter().enumerate() {
        nick_vec[i] = *c;
    }
    u64::from_ne_bytes(nick_vec)
}
