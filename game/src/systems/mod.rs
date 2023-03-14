/// Code for generating the various hand-built systems.
///
/// NOTE: I don't like "system" but here I mean roughly "solar system" not "bevy system", please post your ideas to my coordinates.
///
/// The way I think this should work: A separate package (?) provides tools to build serialized "systems" which you may write to disk if you like.
/// The game can just somehow consume the serialized version, unpack, and proceed as before.
use crate::*;
use std::collections::HashMap;

/// refactor_tags: system, to_client_read, to_client_write, simulation, refactor
pub fn demo_2m2i() -> resources::InitData {
    let mut init_data = resources::InitData::default();
    let position = Vec3::X * 10.0;
    let velocity = Vec3::Y * 0.035;
    let mass = radius_to_mass(1.0);
    init_data.masses.insert(
        0,
        resources::MassInitData {
            inhabitation: Some(components::Inhabitable(None)),
            motion: resources::MassMotion {
                position: position * 1.0,
                velocity: velocity * -1.0,
            },
            color: [1.0, 0.0, 0.0, 1.0],
            mass,
        },
    );
    init_data.masses.insert(
        1,
        resources::MassInitData {
            inhabitation: Some(components::Inhabitable(None)),
            motion: resources::MassMotion {
                position: position * -1.0,
                velocity: velocity * 1.0,
            },
            color: [0.0, 0.0, 1.0, 1.0],
            mass,
        },
    );
    init_data
}

/// refactor_tags: system, to_client_read, to_client_write, simulation, refactor
pub fn get_system(name: &str) -> impl (Fn() -> resources::InitData) {
    match name {
        "demo_2m2i" => demo_2m2i,
        _ => panic!("No such system: {name}"),
    }
}
