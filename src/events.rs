use crate::*;
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Component, Debug)]
pub enum ServerMessage {
    Init(resources::InitData),
    SetGameState(resources::GameState),
    ClientJoined {
        id: u64,
        client_data: wat::ClientData,
    },
    SetPhysicsConfig(physics::PhysicsConfig),
    ClientRotation {
        id: u64,
        rotation: Quat,
    },
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessage {
    Ready,
    Rotation(Quat),
}
