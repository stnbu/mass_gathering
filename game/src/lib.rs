use bevy::ecs::schedule::ShouldRun;
pub use bevy::prelude::*;
pub use bevy_egui::EguiPlugin;
pub use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use bevy_renet::renet::RenetError;
pub use std::f32::consts::TAU;

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

// NOTE: Density is assumed to be 1.0 everywhere, so mass and volume are used interchangably.

pub fn radius_to_mass(radius: f32) -> f32 {
    (3.0 / 4.0) * TAU * radius.powf(3.0)
}

pub fn mass_to_radius(mass: f32) -> f32 {
    ((mass * (4.0 / 3.0)) / TAU).powf(1.0 / 3.0)
}

pub fn scale_to_radius(scale: Vec3) -> f32 {
    // TODO: Should assert that it's a uniform scale.
    scale.x // arbitrary axis
}

pub fn radius_to_scale(radius: f32) -> Vec3 {
    Vec3::splat(radius)
}

pub fn scale_to_mass(scale: Vec3) -> f32 {
    radius_to_mass(scale_to_radius(scale))
}

pub fn mass_to_scale(mass: f32) -> Vec3 {
    radius_to_scale(mass_to_radius(mass))
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
    if cfg!(debug_assertions) {
        let filter = format!("info,wgpu_core=warn,wgpu_hal=off,{package}=debug");
        let level = bevy::log::Level::DEBUG;
        bevy::log::LogPlugin { filter, level }
    } else {
        bevy::log::LogPlugin::default()
    }
}

pub fn panic_on_renet_error(mut renet_error: EventReader<RenetError>) {
    for e in renet_error.iter() {
        error!("{}", e);
    }
}
