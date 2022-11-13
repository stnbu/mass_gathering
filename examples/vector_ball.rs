use bevy::prelude::*;
use mass_gathering::prelude::*;

fn main() {
    App::new()
        .add_plugins(FullGame)
        .add_startup_system(setup)
        .run();
}

const L: f32 = 14.0;
const R: f32 = 1.0;
const B: f32 = 2.0;
const I: f32 = 3.5;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(
            (Cone {
                radius: 2.0 * R,
                height: 2.0 * R,
                ..Default::default()
            })
            .into(),
        ),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        material: materials.add(Color::GREEN.into()),
        ..Default::default()
    });
}
