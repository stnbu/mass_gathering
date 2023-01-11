use crate::*;
use bevy_renet::renet::NETCODE_USER_DATA_BYTES;
use serde::{Deserialize, Serialize};

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
    pub fn to_netcode_user_data(self) -> [u8; NETCODE_USER_DATA_BYTES] {
        let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
        user_data[0] = self.autostart as u8;
        user_data
    }

    pub fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Self {
        let autostart = user_data[0] == 1_u8;
        Self { autostart }
    }
}
