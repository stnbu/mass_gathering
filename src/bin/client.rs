use bevy::prelude::*;
use bevy_renet::{
    renet::{RenetClient, RenetError},
    run_if_client_connected, RenetClientPlugin,
};

use mass_gathering::{
    client_menu, networking::*, spawn_arena_view_camera, systems::spawn_planet, ClientChannel,
    ClientMessages, FullGame, GameState, PhysicsConfig, ServerChannel, ServerMessages,
};

fn main() {
    App::new()
        .add_event::<ClientMessages>()
        .insert_resource(PhysicsConfig { sims_per_frame: 5 })
        .add_startup_system(spawn_arena_view_camera)
        .add_plugin(FullGame::Client)
        //
        .add_plugin(RenetClientPlugin::default())
        .add_system(handle_client_events.with_run_criteria(run_if_client_connected))
        .add_system(send_client_messages.with_run_criteria(run_if_client_connected))
        .add_system(panic_on_renet_error)
        .add_system_set(SystemSet::on_update(GameState::Stopped).with_system(client_menu))
        //
        .run();
}
