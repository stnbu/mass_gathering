use bevy::prelude::App;
use clap::Parser;
use mass_gathering::networking::{
    client::new_renet_client, from_nick, ClientCliArgs, ClientPreferences, FullGameClient,
};
fn main() {
    let args = ClientCliArgs::parse();
    let renet_client = new_renet_client(
        from_nick(&args.nickname),
        ClientPreferences {
            autostart: args.autostart,
        },
    );
    App::new()
        .add_plugin(FullGameClient)
        .insert_resource(renet_client)
        .run();
}
