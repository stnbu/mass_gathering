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
            //
            .add_startup_system(spawn_masses)
            .add_system(panic_on_renet_error)
            .add_system(handle_server_events)
            .add_plugins(DefaultPlugins.set(get_log_plugin("server")))
            .add_plugin(RenetServerPlugin::default())
            .run();
    }
}
