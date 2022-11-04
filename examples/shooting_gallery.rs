use bevy::prelude::*;
use bevy_rapier3d::na::Translation;
use mass_gathering::prelude::*;
fn main() {
    App::new()
        .insert_resource(SpacecraftConfig {
            start_transform: Transform::from_xyz(0.0, 0.0, 40.0),
            ..Default::default()
        })
        .insert_resource(PhysicsConfig {
            sims_per_frame: 5,
            trails: true,
            trail_ttl: 1000 * 600,
        })
        .add_plugins(FullGame)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sun_radius = 8.0;
    let earth_color = Color::rgb(23.0, 57.0, 61.0);
    let sun_color = Color::rgb(244.0, 233.0, 155.0);
    let moon_color = Color::rgb(149.0, 136.0, 132.0);

    // The sun, beautiful
    spawn_planet(
        sun_radius,
        Vec3::ZERO,
        Vec3::ZERO,
        earth_color,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    // The moon, even more beautiful
    spawn_planet(
        1.0,
        Vec3::X * sun_radius * 3.0,
        Vec3::Z * (20.0_f32).powf(0.5) * 0.5,
        moon_color,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}

// use std::collections::HashMap;
// #[derive(Default)]
// struct Breadcrumbs {
//     crumbs: HashMap<String, Vec<Vec3>>, // NO string!
// }

// #[derive(Component)]
// struct Breadcrumb;

// fn spawn_breadcrums(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut breadcrumbs: ResMut<Breadcrumbs>,
// ) {
//     let moon_crumbs = (0..10)
//         .map(|_| {
//             commands
//                 .spawn_bundle(PbrBundle {
//                     mesh: meshes.add(Mesh::from(shape::Icosphere {
//                         radius: 0.01,
//                         ..Default::default()
//                     })),
//                     material: materials.add(Color::GREEN.into()),
//                     visibility: Visibility { is_visible: false },
//                     ..Default::default()
//                 })
//                 .insert(Breadcrumb)
//                 .id()
//         })
//         .collect::<Vec<_>>();
//     breadcrumbs.crumbs.insert("moon".to_string(), moon_crumbs);
// }

// fn update_top_down_view(
//     planets: Query<&Transform, (With<Momentum>, Without<Breadcrumb>)>,
//     spacecraft_query: Query<&Transform, With<Spacecraft>>,
//     mut breadcrumbs_query: Query<
//         (&mut Transform, &mut Visibility),
//         (Without<Momentum>, With<Breadcrumb>),
//     >,
//     mut breadcrumbs: ResMut<Breadcrumbs>,
// ) {
//     let the_spacecraft = spacecraft_query.get_single().unwrap();
//     for (mut transform, mut visibility) in breadcrumbs_query.iter_mut() {
//         //
//     }
// }
