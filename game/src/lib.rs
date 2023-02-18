/// Functionality common to all packages
use bevy::ecs::schedule::ShouldRun;
pub use bevy::prelude::*;
use bevy_renet::renet::RenetError;
pub use std::f32::consts::TAU;

pub mod components;
pub mod events;
pub mod physics;
pub mod resources;
pub mod simulation;
pub mod systems;

pub mod plugins;

pub const PROTOCOL_ID: u64 = 40;
pub const SERVER_IP: &str = "127.0.0.1";
pub const SERVER_PORT: u16 = 5743;

// NOTE: Density is assumed to be 1.0 everywhere, so mass and volume are used interchangably.

/// refactor_tags: UNSET
pub fn radius_to_mass(radius: f32) -> f32 {
    (3.0 / 4.0) * TAU * radius.powf(3.0)
}

/// refactor_tags: UNSET
pub fn mass_to_radius(mass: f32) -> f32 {
    ((mass * (4.0 / 3.0)) / TAU).powf(1.0 / 3.0)
}

/// refactor_tags: UNSET
pub fn scale_to_radius(scale: Vec3) -> f32 {
    // TODO: Should assert that it's a uniform scale.
    scale.x // arbitrary axis
}

/// refactor_tags: UNSET
pub fn radius_to_scale(radius: f32) -> Vec3 {
    Vec3::splat(radius)
}

/// refactor_tags: UNSET
pub fn scale_to_mass(scale: Vec3) -> f32 {
    radius_to_mass(scale_to_radius(scale))
}

/// refactor_tags: UNSET
pub fn mass_to_scale(mass: f32) -> Vec3 {
    radius_to_scale(mass_to_radius(mass))
}

/// refactor_tags: UNSET
pub fn with_gravity(
    game_config: Option<Res<resources::GameConfig>>,
    game_state: Res<State<resources::GameState>>,
) -> ShouldRun {
    if let Some(game_config) = game_config {
        if *game_state.current() == resources::GameState::Running
            && !game_config.physics_config.zerog
        {
            return ShouldRun::Yes;
        }
    }
    ShouldRun::No
}

/// refactor_tags: UNSET
pub fn get_log_plugin(package: &str) -> bevy::log::LogPlugin {
    if cfg!(debug_assertions) {
        let filter = format!("info,wgpu_core=warn,wgpu_hal=off,{package}=debug");
        let level = bevy::log::Level::DEBUG;
        bevy::log::LogPlugin { filter, level }
    } else {
        bevy::log::LogPlugin::default()
    }
}

/// refactor_tags: UNSET
pub fn panic_on_renet_error(mut renet_error: EventReader<RenetError>) {
    for e in renet_error.iter() {
        panic!("Renet error: {e:?}");
    }
}

/// refactor_tags: UNSET
pub fn game_has_started(
    game_config: Option<Res<resources::GameConfig>>,
    game_state: Res<State<resources::GameState>>,
) -> ShouldRun {
    if game_config.is_some() {
        if *game_state.current() == resources::GameState::Running {
            return ShouldRun::Yes;
        }
    }
    ShouldRun::No
}
