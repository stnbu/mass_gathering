pub use bevy::prelude::*;
pub use bevy_egui::EguiPlugin;
pub use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
pub use std::f32::consts::TAU;

use bevy::ecs::schedule::ShouldRun;

pub mod components;
pub mod events;
pub mod physics;
pub mod resources;
pub mod systems;

pub mod plugins;

pub const PROTOCOL_ID: u64 = 32;
pub const SERVER_IP: &str = "127.0.0.1";
pub const SERVER_PORT: u16 = 5743; // FIXME NOTE -- don't change this anymore
pub const SQRT_3: f32 = 1.7320508;

pub fn set_resolution(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    #[cfg(debug_assertions)]
    window.set_resolution(1280.0 / 2.0, 720.0 / 2.0);
    #[cfg(not(debug_assertions))]
    window.set_resolution(1280.0, 720.0);
}

pub fn radius_to_mass(radius: f32) -> f32 {
    (2.0 / 3.0) * TAU * radius.powf(3.0)
}

pub fn mass_to_radius(mass: f32) -> f32 {
    ((mass * (3.0 / 2.0)) / TAU).powf(1.0 / 3.0)
}

pub fn let_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-0.5, -0.3, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 20_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(1.0, -2.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn to_nick(id: u64) -> String {
    let nic_vec: Vec<u8> = id.to_ne_bytes().to_vec();
    String::from_utf8(nic_vec).unwrap() // NOTE includes trailing spaces
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

pub fn with_gravity(
    physics_config: Res<physics::PhysicsConfig>,
    game_state: Res<State<resources::GameState>>,
) -> ShouldRun {
    if *game_state.current() == resources::GameState::Running && !physics_config.zerog {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn get_log_plugin(package: &str) -> bevy::log::LogPlugin {
    #[cfg(debug_assertions)]
    {
        let filter = format!("info,wgpu_core=warn,wgpu_hal=off,{package}=debug");
        let level = bevy::log::Level::DEBUG;
        bevy::log::LogPlugin { filter, level }
    }
    #[cfg(not(debug_assertions))]
    {
        bevy::log::LogPlugin::default()
    }
}
