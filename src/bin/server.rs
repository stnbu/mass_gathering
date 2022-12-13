use bevy::prelude::*;
use clap::Parser;
use mass_gathering::{networking::*, systems::*};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    speed: u32,
}

fn main() {
    let _speed = Args::parse().speed;
    App::new()
        .insert_resource(testing_no_unhinhabited())
        .add_startup_system(spawn_arena_view_camera)
        .add_plugin(FullGame::Server)
        .run();
}
