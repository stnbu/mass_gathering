pub use bevy::prelude::*;
pub use bevy_egui::EguiPlugin;
pub use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
pub use std::f32::consts::TAU;

use bevy_renet::renet::RenetError;

pub mod client;
pub mod components;
pub mod events;
pub mod physics;
pub mod resources;
pub mod server;
pub mod systems;

pub const PROTOCOL_ID: u64 = 29;
pub const SERVER_IP: &str = "127.0.0.1";
pub const SERVER_PORT: u16 = 5743; // FIXME NOTE -- don't change this anymore
pub const CHANNEL_RELIABLE: u8 = 0;

pub struct Core;
impl Plugin for Core {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            debug!("DEBUG LEVEL LOGGING ! !");
            app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=off,mass_gathering=debug,mass_gathering::networking=debug".into(),
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
        app.insert_resource(resources::MassIDToEntity::default());
        app.add_event::<events::ClientMessage>();
        app.add_event::<events::ServerMessage>();
        app.init_resource::<resources::GameConfig>();
        app.add_state(resources::GameState::Stopped);
        app.add_system_set(
            SystemSet::on_update(resources::GameState::Running)
                .with_system(client::control)
                .with_system(client::handle_projectile_engagement)
                .with_system(client::handle_projectile_fired)
                .with_system(client::move_projectiles)
                .with_system(client::rotate_client_inhabited_mass),
        );
        app.add_plugin(EguiPlugin);
        app.add_startup_system(let_light);
        app.add_system(bevy::window::close_on_esc);
        app.insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            ..Default::default()
        });
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

pub struct Spacetime;
impl Plugin for Spacetime {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .init_resource::<physics::PhysicsConfig>()
            .add_event::<physics::MassCollisionEvent>()
            .add_event::<physics::DespawnMassEvent>()
            .add_system_set(
                SystemSet::on_update(resources::GameState::Running)
                    .with_system(physics::handle_despawn_mass)
                    .with_system(physics::freefall.before(physics::handle_despawn_mass))
                    .with_system(
                        physics::handle_mass_collisions.before(physics::handle_despawn_mass),
                    )
                    .with_system(physics::merge_masses.before(physics::handle_despawn_mass)),
            );
    }
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

pub fn panic_on_renet_error(mut renet_error: EventReader<RenetError>) {
    for e in renet_error.iter() {
        error!("{}", e);
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
