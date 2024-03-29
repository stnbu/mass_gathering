use bevy::prelude::*;
use mass_gathering::prelude::*;

fn main() {
    App::new()
        .add_plugins(FullGame)
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(SpacecraftConfig {
            stereo_enabled: false,
            stereo_iod: 2.0,
            start_transform: Transform::from_xyz(0.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            // FIXME: all this got miscalibrated after much refactoring. Not sure what this value sould be.
            //        mostly the problem is fire-control. you shoot of so many. maybe other causes..?
            //        ALSO need to scale vectorball vector by mass. In any case, they are pancakey here.
            //        @ba62fea97853d464bd869d9415bc04b78ecbf723
            impact_magnitude: 0.8,
            ..Default::default()
        })
        .insert_resource(PhysicsConfig {
            sims_per_frame: 1,
            trails: true,
            trail_ttl: 10_000,
        })
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
    //let sun_color = Color::rgb(244.0, 233.0, 155.0);
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
        Vec3::Z * 1.588,
        moon_color,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
