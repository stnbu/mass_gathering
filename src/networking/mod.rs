use bevy::prelude::*;
use bevy_renet::{
    renet::{
        ChannelConfig, ReliableChannelConfig, RenetError, NETCODE_KEY_BYTES,
        NETCODE_USER_DATA_BYTES,
    },
    run_if_client_connected, RenetClientPlugin, RenetServerPlugin,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

pub mod client;
pub mod server;
use crate::{ui, Core, GameState, PhysicsConfig, Spacetime};

pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"dwxx_SERxx24,0)cs2@66#vxo0s5np{_";
pub const PROTOCOL_ID: u64 = 23;
pub const SERVER_ADDR: &str = "127.0.0.1";
pub const PORT_NUMBER: u16 = 5737;

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct MassInitData {
    pub position: Vec3,
    pub velocity: Vec3,
    pub color: Color,
    pub radius: f32,
}

#[derive(Default, Serialize, Deserialize, Resource, Debug)]
pub struct InitData {
    pub masses: HashMap<u64, MassInitData>,
}

impl Clone for InitData {
    fn clone(&self) -> Self {
        let mut masses = HashMap::new();
        masses.extend(&self.masses);
        Self { masses }
    }

    fn clone_from(&mut self, source: &Self) {
        let mut masses = HashMap::new();
        masses.extend(&source.masses);
        self.masses = masses;
    }
}

pub fn panic_on_renet_error(mut renet_error: EventReader<RenetError>) {
    for e in renet_error.iter() {
        panic!("{}", e);
    }
}

pub enum ServerChannel {
    ServerMessages,
}

#[derive(Serialize, Deserialize, Component, Debug)]
pub enum ServerMessages {
    Init(InitData),
    SetGameState(GameState),
    ClientJoined { id: u64, client_data: ClientData },
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

#[derive(Component)]
pub struct MassID(pub u64);

#[derive(Resource, Default)]
pub struct MapMassIDToEntity(HashMap<u64, Entity>);

pub fn spawn_arena_view_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        ..Default::default()
    });
}

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

#[derive(Serialize, Deserialize, Component, Debug, Copy, Clone)]
pub struct ClientPreferences {
    pub autostart: bool,
}

#[derive(Serialize, Deserialize, Component, Debug, Copy, Clone)]
pub struct ClientData {
    pub preferences: ClientPreferences,
    pub inhabited_mass_id: u64,
}

impl ClientPreferences {
    fn to_netcode_user_data(&self) -> [u8; NETCODE_USER_DATA_BYTES] {
        let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
        user_data[0] = self.autostart as u8;
        user_data
    }

    fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Self {
        let autostart = user_data[0] == 1_u8;
        Self { autostart }
    }
}

#[derive(Default, Resource, Debug)]
pub struct Lobby {
    pub clients: HashMap<u64, ClientData>,
}

pub enum FullGame {
    Client,
    Server,
}

impl Plugin for FullGame {
    fn build(&self, app: &mut App) {
        app.add_plugin(Core);
        app.add_plugin(Spacetime);
        app.insert_resource(Lobby::default());
        app.insert_resource(PhysicsConfig { sims_per_frame: 5 });
        app.insert_resource(MapMassIDToEntity::default());
        match self {
            Self::Client => {
                app.add_system_set(
                    SystemSet::on_update(GameState::Waiting).with_system(ui::client_waiting_screen),
                );
                app.add_system_set(
                    SystemSet::on_update(GameState::Stopped).with_system(ui::client_menu_screen),
                );
                app.add_system_set(
                    SystemSet::on_update(GameState::Running).with_system(client::control),
                );

                app.add_plugin(RenetClientPlugin::default());
                app.add_system(
                    client::handle_client_events.with_run_criteria(run_if_client_connected),
                );
                app.add_system(
                    client::send_client_messages.with_run_criteria(run_if_client_connected),
                );
                app.add_system(panic_on_renet_error);
                app.add_system(client::set_window_title);
            }
            Self::Server => {
                app.add_plugin(RenetServerPlugin::default());
                app.insert_resource(server::new_renet_server());
                app.add_system(server::handle_server_events);
                app.add_system(server::set_window_title);
            }
        }
    }
}
