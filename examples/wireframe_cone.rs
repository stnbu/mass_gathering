use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::{render_resource::WgpuFeatures, settings::WgpuSettings};
use mass_gathering::prelude::*;

fn main() {
    App::new()
        .add_plugins(FullGame)
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugin(WireframePlugin)
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(SpacecraftConfig {
            start_transform: Transform::from_xyz(0.0, 0.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            impact_magnitude: 100.0,
            projectile_radius: 0.05,
            ..Default::default()
        })
        .insert_resource(PhysicsConfig {
            sims_per_frame: 10,
            trails: true,
            trail_ttl: 20_000,
        })
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = false;

    // this repo stuff
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Cone::default().into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(Color::GREEN.into()),
            ..Default::default()
        })
        .insert(Wireframe);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Cylinder::default().into()),
            transform: Transform::from_xyz(0.0, 6.0, 0.0),
            material: materials.add(Color::YELLOW.into()),
            ..Default::default()
        })
        .insert(Wireframe);

    // bevy stuff
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(shape::Box::new(0.8, 1.7, 3.3).into()),
            transform: Transform::from_xyz(6.0, 0.0, 0.0),
            material: materials.add(Color::BLUE.into()),
            ..Default::default()
        })
        .insert(Wireframe);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(
                (shape::Icosphere {
                    radius: 2.0,
                    ..Default::default()
                })
                .into(),
            ),
            transform: Transform::from_xyz(-6.0, 0.0, 0.0),
            material: materials.add(Color::RED.into()),
            ..Default::default()
        })
        .insert(Wireframe);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(shape::Box::new(0.8, 1.7, 3.3).into()),
            transform: Transform::from_xyz(6.0, 0.0, 0.0),
            material: materials.add(Color::BLUE.into()),
            ..Default::default()
        })
        .insert(Wireframe);
}
