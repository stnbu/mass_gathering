/// Plugin(s) required to implement a server binary (executable).
use crate::*;
use bevy_egui::EguiPlugin;
use clap::Parser;

#[derive(Default)]
/// refactor_tags: UNSET
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK));
        app.add_plugins(DefaultPlugins.set(get_log_plugin("client")));
        app.add_system(simulation::handle_game_config_insertion);
        app.add_system(handle_set_game_state);
        app.add_system(handle_set_game_config);
        app.add_system(visualize_masses);
        app.add_system(send_messages_to_server);
        app.add_system(receive_messages_from_server);
        app.add_system(panic_on_renet_error);
        app.add_plugin(EguiPlugin);
        app.insert_resource(resources::UiState::default());
        app.add_system(info_text);
        app.add_system(set_active_camera);
        app.add_system(set_ui_state);
        app.add_system(position_objective_camera);
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(game_has_started)
                .with_system(simulation::handle_projectile_fired)
                .with_system(simulation::move_projectiles)
                .with_system(simulation::handle_projectile_collision)
                .with_system(simulation::rotate_inhabitable_masses)
                //
                .with_system(control)
                .with_system(handle_projectile_engagement)
                .with_system(visualize_projectiles),
        );

        app.add_system(bevy::window::close_on_esc);
        app.add_system(set_window_title);
        app.add_startup_system(spawn_cameras);
        app.add_startup_system(set_resolution);
        app.add_startup_system(let_light);
        app.add_plugin(RenetClientPlugin::default());
        let args = ClientCliArgs::parse();
        let player = components::Player::from(&args.nickname);
        app.insert_resource(player);
        let address = args.address.clone();
        let renet_client = new_renet_client(player, address);
        app.insert_resource(renet_client);
    }
}
