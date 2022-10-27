use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use mass_gathering::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE * 0.8))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(collision_events)
        .add_system(freefall)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(print_planet_specs)
        .run();
}

struct Merged(bool);

impl Default for Merged {
    fn default() -> Self {
        Merged(false)
    }
}

fn print_planet_specs(query: Query<(Entity, &Transform, &Momentum)>, mut merged: Local<Merged>) {
    if !merged.0 {
        for (entity, transform, momentum) in query.iter() {
            println!("Entity {:?}", entity);
            println!("  {transform:?}");
            println!("  {momentum:?}");
        }
        if query.iter().len() < 2 {
            merged.0 = true;
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec3::ZERO;

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 40.0).looking_at(-Vec3::Z, Vec3::Y),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 30.0, 40.0),
        point_light: PointLight {
            intensity: 250000.0,
            range: 1000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    spawn_planet(
        5.01,
        Vec3::X * 10.0,
        Vec3::ZERO,
        Color::RED,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    spawn_planet(
        5.0,
        Vec3::X * -10.0,
        Vec3::ZERO,
        Color::BLUE,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
