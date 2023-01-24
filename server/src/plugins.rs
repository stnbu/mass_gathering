use crate::*;
use bevy_renet::RenetServerPlugin;
use clap::Parser;

#[derive(Default)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let args = resources::ServerCliArgs::parse();
        let system = args.system.clone();
        let address = args.address.clone();
        let init_data = systems::get_system(&system)();
        app
            //
            .insert_resource(new_renet_server(address))
            .insert_resource(args)
            .insert_resource(resouces::GameConfig {
                init_data,
                ..Default::default()
            })
            .init_resource::<UnassignedMasses>()
            .add_startup_system(populate_unassigned_masses)
            .add_startup_system(setup_physics)
            //.with_system(panic_on_renet_error)
            .add_system(panic_on_renet_error)
            .add_system(handle_server_events)
            // // this causes high cpu usage
            // .add_plugins(MinimalPlugins)
            // .add_plugin(get_log_plugin("server"))
            // // this causes immediate exit
            // .add_plugins(
            //     DefaultPlugins
            //         .set(WindowPlugin {
            //             add_primary_window: false,
            //             ..Default::default()
            //         })
            //         .set(get_log_plugin("server")),
            // )
            // this causes a window, but it's better than cpu hogging
            .add_plugins(DefaultPlugins.set(get_log_plugin("server")))
            .add_plugin(RenetServerPlugin::default())
            .run();
    }
}
