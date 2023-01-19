use bevy::{app::ScheduleRunnerPlugin, time::TimePlugin};
use bevy_renet::RenetServerPlugin;
use clap::Parser;
use game::*;

fn main() {
    let args = resources::ServerCliArgs::parse();
    let system = args.system.clone();
    let address = args.address.clone();
    let mut app = App::new();
    app.insert_resource(resources::Lobby::default())
        .add_state(resources::GameState::Stopped)
        .add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .insert_resource(args)
        .init_resource::<server::UnassignedMasses>()
        .add_startup_system(server::populate_unassigned_masses)
        .add_startup_system(server::setup_physics)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(server::new_renet_server(address))
        .add_system(server::handle_server_events)
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
