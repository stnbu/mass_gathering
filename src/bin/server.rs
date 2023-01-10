use bevy::{app::ScheduleRunnerPlugin, prelude::*, time::TimePlugin};
use bevy_renet::RenetServerPlugin;
use clap::Parser;
use mass_gathering::{networking, systems, GameConfig, GameState};

fn main() {
    let args = networking::ServerCliArgs::parse();
    let system = args.system.clone();
    let mut app = App::new();
    app.insert_resource(networking::Lobby::default())
        .init_resource::<GameConfig>()
        .add_state(GameState::Stopped)
        .add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .insert_resource(args)
        .init_resource::<networking::server::UnassignedMasses>()
        .add_startup_system(networking::server::populate_unassigned_masses)
        .add_startup_system(networking::server::setup_physics)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(networking::server::new_renet_server())
        .add_system(networking::server::handle_server_events)
        .insert_resource(systems::get_system(&system)());

    #[cfg(debug_assertions)]
    {
        debug!("DEBUG LEVEL LOGGING ! !");
        app.add_plugin(bevy::log::LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=off,mass_gathering=debug,mass_gathering::networking=debug".into(),
                level: bevy::log::Level::DEBUG,
            });
    }

    app.run();
}
