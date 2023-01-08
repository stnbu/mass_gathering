use bevy::{app::ScheduleRunnerPlugin, prelude::*, time::TimePlugin};
use bevy_renet::RenetServerPlugin;
use clap::Parser;
use mass_gathering::{networking::*, systems::get_system, GameConfig, GameState};

fn main() {
    let args = ServerCliArgs::parse();
    let system = args.system.clone();
    let mut app = App::new();
    app.insert_resource(Lobby::default())
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
        .insert_resource(get_system(&system)());

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
