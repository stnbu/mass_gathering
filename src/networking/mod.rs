use bevy::{prelude::*};
use bevy_renet::{
    renet::{
        ChannelConfig, ReliableChannelConfig, RenetError, NETCODE_KEY_BYTES,
        NETCODE_USER_DATA_BYTES,
    },
    run_if_client_connected, RenetClientPlugin,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

pub mod client;
pub mod server;
use crate::{
    set_window_title, ui, Core, GameState, InitData,
    PhysicsConfig, Spacetime,
};

pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"dwxx_SERxx24,3)cs2@66#vxo0s5np{_";
pub const PROTOCOL_ID: u64 = 24;
pub const SERVER_ADDR: &str = "127.0.0.1";
pub const PORT_NUMBER: u16 = 5738;

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
    SetPhysicsConfig(PhysicsConfig),
    ClientRotation { id: u64, rotation: Quat },
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

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessages {
    Ready,
    Rotation(Quat),
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

use clap::Parser;

#[derive(Parser, Resource)]
pub struct ClientCliArgs {
    #[arg(short, long, default_value_t = ("NICK").to_string())]
    pub nickname: String,
}
#[derive(Parser, Resource)]
pub struct ServerCliArgs {
    #[arg(short, long, default_value_t = 10)]
    pub speed: u32,
}

pub struct FullGameClient;

impl Plugin for FullGameClient {
    fn build(&self, app: &mut App) {
        app.add_plugin(Core);
        app.insert_resource(Lobby::default());
        app.add_plugin(Spacetime);
        app.insert_resource(ClientCliArgs::parse());
        app.add_system_set(
            SystemSet::on_update(GameState::Waiting).with_system(ui::client_waiting_screen),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Stopped).with_system(ui::client_menu_screen),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Running).with_system(client::send_rotation_to_server),
        );

        app.add_plugin(RenetClientPlugin::default());
        app.add_system(client::handle_client_events.with_run_criteria(run_if_client_connected));
        app.add_system(client::send_client_messages.with_run_criteria(run_if_client_connected));
        app.add_system(panic_on_renet_error);
        app.add_system(set_window_title);
    }
}
