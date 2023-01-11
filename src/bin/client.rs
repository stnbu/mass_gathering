use clap::Parser;
use mass_gathering::*;

fn main() {
    let args = resources::ClientCliArgs::parse();
    let renet_client = client::new_renet_client(
        from_nick(&args.nickname),
        resources::ClientPreferences {
            autostart: args.autostart,
        },
    );
    App::new()
        .add_plugin(client::ClientPlugin)
        .insert_resource(renet_client)
        .run();
}
