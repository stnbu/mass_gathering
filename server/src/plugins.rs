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
            .add_plugins(MinimalPlugins)
            // FIXME: As per https://github.com/dimforge/bevy_rapier/issues/296
            // manually adding some stuff. Is there a problem with this? Can/should
            // I use the `headless` feature of `bevy_rapier`?
            .add_plugin(AssetPlugin::default())
            .add_asset::<Mesh>()
            .add_asset::<Scene>()
            .add_plugin(get_log_plugin("server"))
            .insert_resource(new_renet_server(address))
            .insert_resource(resources::GameConfig {
                physics_config: resources::PhysicsConfig { speed, zerog },
                init_data: systems::get_system(&args.system)(),
                ..Default::default()
            })
            .add_startup_system(spawn_masses)
            .add_system(panic_on_renet_error)
            .add_system(handle_server_events)
            .add_plugin(RenetServerPlugin::default())
            .run();
    }
}
