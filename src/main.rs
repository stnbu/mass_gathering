use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_browser_resize);

    app.insert_resource(ClearColor(Color::rgb(
        0xF9 as f32 / 255.0,
        0xF9 as f32 / 255.0,
        0xFF as f32 / 255.0,
    )))
    .insert_resource(Msaa::default())
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system(setup_graphics)
    .add_startup_system(setup_physics)
    .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 200.0, 0.0),
        ..default()
    });
}

use rand::Rng;
pub fn setup_physics(mut commands: Commands) {
    for y in [500.0, 400.0, 300.0, 200.0, 100.0, 0.0, -100.0, -200.0] {
        for x in [
            -400.0, -300.0, -200.0, -100.0, 0.0, 100.0, 200.0, 300.0, 400.0,
        ] {
            commands
                .spawn_bundle(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
                .insert(RigidBody::Dynamic)
                .insert(LockedAxes::TRANSLATION_LOCKED)
                .insert(Collider::cuboid(48.0, 5.0))
                .insert(Friction::new(0.05));
        }
    }

    let mut rng = rand::thread_rng();
    for x in [
        -420.0, -320.0, -220.0, -120.0, -20.0, 80.0, 180.0, 280.0, 380.0,
    ] {
        let wiggle: u8 = rng.gen::<u8>() % 20;
        commands
            .spawn_bundle(TransformBundle::from(
                Transform::from_xyz(x + wiggle as f32, 800.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(1.0)),
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(3.0, 3.0))
            .insert(ColliderMassProperties::Density(0.1 * wiggle as f32))
            .insert(Friction::new(1.0 / wiggle as f32));
    }
}

#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let wasm_window = web_sys::window().unwrap();
    let (target_width, target_height) = (
        wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
        wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );

    if window.width() != target_width || window.height() != target_height {
        window.set_resolution(target_width, target_height);
    }
}
