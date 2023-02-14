/// The server binary (executable)
use bevy::prelude::App;

fn main() {
    App::new()
        .add_plugin(game::plugins::CorePlugin::default())
        .add_plugin(server::plugins::ServerPlugin::default())
        .run();
}
