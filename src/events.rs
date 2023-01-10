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
pub enum ClientMessage {
    Ready,
    Rotation(Quat),
}
