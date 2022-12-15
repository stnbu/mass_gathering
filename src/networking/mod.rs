use bevy::{app::ScheduleRunnerPlugin, prelude::*, time::TimePlugin};
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
use crate::{
    inhabitant, set_window_title, systems, ui, ClientCore, Core, GameState, InitData,
    MassIDToEntity, PhysicsConfig, Spacetime,
};

pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"dwxx_SERxx24,0)cs2@66#vxo0s5np{_";
pub const PROTOCOL_ID: u64 = 23;
pub const SERVER_ADDR: &str = "127.0.0.1";
pub const PORT_NUMBER: u16 = 5737;

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

pub enum FullGame {
    Client,
    Server,
    Standalone,
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

impl Plugin for FullGame {
    fn build(&self, app: &mut App) {
        app.add_plugin(Core);
        app.insert_resource(Lobby::default());
        app.insert_resource(MassIDToEntity::default());
        match self {
            Self::Client => {
                app.add_plugin(ClientCore);
                app.add_plugin(Spacetime);
                app.insert_resource(ClientCliArgs::parse());
                app.add_system_set(
                    SystemSet::on_update(GameState::Waiting).with_system(ui::client_waiting_screen),
                );

                app.add_system_set(
                    SystemSet::on_update(GameState::Stopped).with_system(ui::client_menu_screen),
                );
                app.add_system_set(
                    SystemSet::on_update(GameState::Running)
                        .with_system(client::send_rotation_to_server),
                );

                app.add_plugin(RenetClientPlugin::default());
                app.add_system(
                    client::handle_client_events.with_run_criteria(run_if_client_connected),
                );
                app.add_system(
                    client::send_client_messages.with_run_criteria(run_if_client_connected),
                );
                app.add_system(panic_on_renet_error);
                app.add_system(set_window_title);
            }
            Self::Server => {
                app.add_plugin(CorePlugin::default());
                app.add_plugin(TimePlugin::default());
                app.add_plugin(ScheduleRunnerPlugin::default());
                app.insert_resource(ServerCliArgs::parse());
                app.init_resource::<server::UnassignedMasses>();
                app.add_startup_system(server::populate_unassigned_masses);
                app.add_startup_system(server::setup_physics);
                app.add_plugin(RenetServerPlugin::default());
                app.insert_resource(server::new_renet_server());
                app.add_system(server::handle_server_events);
            }
            Self::Standalone => {
                app.add_plugin(ClientCore);
                app.add_plugin(Spacetime);
                app.insert_resource(systems::testing_no_unhinhabited());
                app.add_startup_system(setup_standalone);
            }
        }
    }
}

//
// FIXME
//
fn setup_standalone(
    init_data: Res<InitData>,
    mut mass_to_entity_map: ResMut<MassIDToEntity>,
    mut game_state: ResMut<State<GameState>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // FIXME: some logic overlap with ClientJoined handler
    *mass_to_entity_map = init_data
        .clone()
        .init(&mut commands, &mut meshes, &mut materials);

    let mut mass_id_ = None;
    for (mass_id, mass_init_data) in init_data.masses.iter() {
        if mass_init_data.inhabitable {
            mass_id_ = Some(mass_id);
            break;
        }
    }
    let mass_id = mass_id_.unwrap();
    let inhabited_mass = mass_to_entity_map.0.get(mass_id).unwrap();
    let mut inhabited_mass_commands = commands.entity(*inhabited_mass);
    inhabited_mass_commands.insert(inhabitant::ClientInhabited);
    inhabited_mass_commands.despawn_descendants();
    debug!("Appending camera to inhabited mass {inhabited_mass:?}");
    inhabited_mass_commands.with_children(|child| {
        child.spawn(Camera3dBundle::default());
    });
    let _ = game_state.overwrite_set(GameState::Running);
}
