use crate::*;
use bevy_renet::RenetServerPlugin;
use clap::Parser;

#[derive(Default)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let log_plugin = get_log_plugin("server");
        #[cfg(feature = "windows")]
        {
            app.add_plugins(DefaultPlugins.set(log_plugin));
        }
        #[cfg(not(feature = "windows"))]
        {
            app.add_plugins(MinimalPlugins);
            // https://github.com/dimforge/bevy_rapier/issues/296
            app.add_plugin(AssetPlugin::default());
            app.add_asset::<Mesh>();
            app.add_asset::<Scene>();
            app.add_plugin(log_plugin);
        }

        let args = ServerCliArgs::parse();
        let address = args.address.clone();
        let zerog = args.zerog;
        let speed = args.speed;

        let physics_config = resources::PhysicsConfig { speed, zerog };
        let init_data = systems::get_system(&args.system)();
        let game_config = resources::GameConfig {
            physics_config,
            init_data,
            ..Default::default()
        };

        app
            //
            .insert_resource(new_renet_server(address))
            .insert_resource(game_config)
            //
            .add_plugin(game::plugins::SimulationPlugin::default())
            .add_event::<simulation::FromSimulation>()
            .add_system_set(
                SystemSet::on_update(resources::GameState::Running)
                    .with_system(simulation::handle_projectile_fired)
                    .with_system(simulation::move_projectiles)
                    .with_system(simulation::handle_projectile_collision)
                    .with_system(simulation::rotate_inhabitable_masses),
            )
            .add_system(panic_on_renet_error)
            .add_system(handle_server_events)
            .add_plugin(RenetServerPlugin::default());
        //app.run();
    }
}
