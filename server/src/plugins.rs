use crate::*;
use bevy_renet::RenetServerPlugin;
use clap::Parser;

#[derive(Default)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let args = ServerCliArgs::parse();
        let address = args.address.clone();
        let zerog = args.zerog;
        let speed = args.speed;

        app
            //
            .insert_resource(new_renet_server(address))
            .insert_resource(resources::GameConfig {
                physics_config: resources::PhysicsConfig { speed, zerog },
                init_data: systems::get_system(&args.system)(),
                ..Default::default()
            })
            .add_system(panic_on_renet_error)
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
