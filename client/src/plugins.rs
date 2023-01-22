use crate::*;
use clap::Parser;

#[derive(Default)]
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(get_log_plugin("client")));
        app.insert_resource(ClearColor(Color::BLACK));
        app.add_system_set(
            SystemSet::on_update(resources::GameState::Running)
                .with_system(control)
                .with_system(handle_projectile_engagement)
                .with_system(handle_projectile_fired)
                .with_system(move_projectiles)
                .with_system(handle_projectile_collision)
                .with_system(rotate_inhabitable_masses),
        );
        app.add_plugin(EguiPlugin);
        app.add_startup_system(set_resolution);
        app.add_startup_system(let_light);
        app.add_system(bevy::window::close_on_esc);
        app.add_system(set_window_title);
        app.add_system_set(
            SystemSet::on_update(resources::GameState::Waiting).with_system(client_waiting_screen),
        );
        app.add_system_set(
            SystemSet::on_update(resources::GameState::Running)
                .with_run_criteria(run_if_client_connected)
                .with_system(send_messages_to_server)
                .with_system(process_to_client_events)
                .with_system(receive_messages_from_server)
                .with_system(animate_explosions)
                .with_system(panic_on_renet_error),
        );
        app.add_plugin(RenetClientPlugin::default());

        let args = resources::ClientCliArgs::parse();
        let address = args.address.clone();
        let renet_client = new_renet_client(from_nick(&args.nickname), address);
        app.insert_resource(renet_client);
    }
}
