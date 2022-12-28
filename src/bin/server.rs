use bevy::{app::ScheduleRunnerPlugin, prelude::*, time::TimePlugin};
use bevy_renet::RenetServerPlugin;
use clap::Parser;
use mass_gathering::{inhabitant, networking::*, systems::get_system, GameConfig, GameState};

fn main() {
    let args = ServerCliArgs::parse();
    let system = args.system.clone();
    App::new()
        .insert_resource(Lobby::default())
        .add_event::<inhabitant::ClientRotation>()
        .init_resource::<GameConfig>()
        .add_state(GameState::Stopped)
        .add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .insert_resource(args)
        .init_resource::<server::UnassignedMasses>()
        .add_startup_system(server::populate_unassigned_masses)
        .add_startup_system(server::setup_physics)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(server::new_renet_server())
        .add_system(server::handle_server_events)
        .insert_resource(get_system(&system)())
        .run();
}
