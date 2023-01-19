use clap::Parser;
use game::*;

fn main() {
    let args = resources::ClientCliArgs::parse();
    let address = args.address.clone();
    let renet_client = client::new_renet_client(
        from_nick(&args.nickname),
        resources::ClientPreferences {
            autostart: args.autostart,
        },
        address,
    );
    App::new()
        .add_plugin(client::ClientPlugin)
        .insert_resource(renet_client)
        .run();
}
