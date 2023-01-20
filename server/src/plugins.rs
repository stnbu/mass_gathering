use crate::*;
use bevy::{app::ScheduleRunnerPlugin, time::TimePlugin};
use bevy_renet::RenetServerPlugin;
use clap::Parser;

#[derive(Default)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let args = resources::ServerCliArgs::parse();
        let system = args.system.clone();
        let address = args.address.clone();
        app
            // xx
            .insert_resource(new_renet_server(address))
            .insert_resource(args)
            .insert_resource(systems::get_system(&system)())
            .init_resource::<UnassignedMasses>()
            .add_startup_system(populate_unassigned_masses)
            .add_startup_system(setup_physics)
            .add_system(handle_server_events)
            // yy
            // Theirs
            .add_plugin(CorePlugin::default())
            //.add_plugin(PbrPlugin::default())
            .add_plugin(TimePlugin::default())
            .add_plugin(ScheduleRunnerPlugin::default())
            .add_plugin(RenetServerPlugin::default())
            .run();
    }
}
