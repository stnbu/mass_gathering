use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use std::f32::consts::PI;

pub mod physics;
pub use physics::*;

pub mod systems;

pub struct FullGame;

impl PluginGroup for FullGame {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(Core).add(Spacetime)
    }
}

pub fn let_light(mut commands: Commands) {
    // TODO: These are to be messed with.
    const NORMAL_BIAS: f32 = 0.61;
    const SHADOW_BIAS: f32 = 0.063;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            shadow_normal_bias: NORMAL_BIAS,
            shadow_depth_bias: SHADOW_BIAS,
            ..default()
        },
        // TODO: figure out what _translation_ means for directional
        transform: Transform::from_xyz(-500000.0, -500000.0, 0.0),
        ..default()
    });
}

pub struct Spacetime;

impl Plugin for Spacetime {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .init_resource::<PhysicsConfig>()
            .add_event::<DeltaEvent>()
            .add_event::<PlanetCollisionEvent>()
            .add_event::<DespawnPlanetEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(handle_despawn_planet)
                    .with_system(signal_freefall_delta.before(handle_despawn_planet))
                    .with_system(handle_freefall.before(handle_despawn_planet))
                    .with_system(handle_planet_collisions.before(handle_despawn_planet))
                    .with_system(merge_planets.before(handle_despawn_planet)),
            );
    }
}

pub struct Core;

impl Plugin for Core {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            debug!("DEBUG LEVEL LOGGING ! !");
            app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=off,mass_gathering=debug".into(),
                level: bevy::log::Level::DEBUG,
            }));
        }

        #[cfg(not(debug_assertions))]
        {
            error!("We have no logging, and yet you SEE this message...?");
            // FIXME: num-triangles on a mesh is a different thing
            app.insert_resource(Msaa { samples: 4 });
            app.add_plugins(DefaultPlugins);
        }

        app.add_startup_system(let_light);
        app.add_system(bevy::window::close_on_esc);

        app.add_state(GameState::Stopped)
            .add_startup_system(disable_rapier_gravity)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum GameState {
    Running,
    Stopped,
}

fn disable_rapier_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec3::ZERO;
}

pub fn radius_to_mass(radius: f32) -> f32 {
    (4.0 / 3.0) * PI * radius.powf(3.0)
}

pub fn mass_to_radius(mass: f32) -> f32 {
    ((mass * (3.0 / 4.0)) / PI).powf(1.0 / 3.0)
}

// Additions while _trying to use_ renet for an actual "mass gathering"
// ====
use std::collections::HashMap;
#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct PlanetInitData {
    pub position: Vec3,
    pub velocity: Vec3,
    pub color: Color,
    pub radius: f32,
}

#[derive(Default, Serialize, Deserialize, Resource)]
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

//
use bevy_renet::renet::{
    ChannelConfig, ReliableChannelConfig, RenetConnectionConfig, NETCODE_KEY_BYTES,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"dwxe_SERxx29e0)cs2@66#vxo0s5np{_";
pub const PROTOCOL_ID: u64 = 13;
pub const SERVER_ADDR: &str = "192.168.1.43";
pub const PORT_NUMBER: u16 = 5243;

pub enum ServerChannel {
    ServerMessages,
}

#[derive(Serialize, Deserialize, Component)]
pub enum ServerMessages {
    SendInitData(InitData),
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

pub fn spawn_server_view_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(-Vec3::Z, Vec3::Y),
        ..Default::default()
    });
}
