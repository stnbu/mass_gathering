use bevy::prelude::App;

fn main() {
    App::new()
        .add_plugin(game::plugins::CorePlugin::default())
        .add_plugin(game::plugins::SimulationPlugin::default())
        .add_plugin(client::plugins::ClientPlugin::default())
        .run();
}
