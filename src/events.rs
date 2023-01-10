use bevy::prelude::*;
use bevy_renet::{
    renet::{RenetError, NETCODE_KEY_BYTES, NETCODE_USER_DATA_BYTES},
    run_if_client_connected, RenetClientPlugin,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{physics::PhysicsConfig, ui, Core, GameState, InitData, Spacetime};

#[derive(Serialize, Deserialize, Component, Debug)]
pub enum ServerMessage {
    Init(InitData),
    SetGameState(GameState),
    ClientJoined { id: u64, client_data: ClientData },
    SetPhysicsConfig(PhysicsConfig),
    ClientRotation { id: u64, rotation: Quat },
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessages {
    Ready,
    Rotation(Quat),
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
    fn to_netcode_user_data(self) -> [u8; NETCODE_USER_DATA_BYTES] {
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

#[derive(Parser, Resource)]
pub struct ClientCliArgs {
    #[arg(long)]
    pub nickname: String,
    #[arg(long, default_value_t = true)]
    pub autostart: bool,
}
#[derive(Parser, Resource)]
pub struct ServerCliArgs {
    #[arg(long, default_value_t = 1)]
    pub speed: u32,
    #[arg(long, default_value_t = ("").to_string())]
    pub system: String,
}
