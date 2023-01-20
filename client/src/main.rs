use bevy::prelude::App;
use clap::Parser;
use game::from_nick;

fn main() {
    let args = game::resources::ClientCliArgs::parse();
    let address = args.address.clone();
    let renet_client = client::new_renet_client(from_nick(&args.nickname), address);
    App::new()
        .add_plugin(game::plugins::CorePlugin::default())
        .add_plugin(client::plugins::ClientPlugin::default())
        .insert_resource(renet_client)
        .run();
}
