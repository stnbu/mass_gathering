use bevy::prelude::*;
use std::collections::HashSet;
use std::f32::consts::TAU;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(LocalPathCurvature::default())
        .add_state(AppState::Startup)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(rocket_forward)
                .with_system(steer),
        )
        .add_system(bevy::window::close_on_esc) // "or prototyping" -- unclean shutdown
        .add_system(window_focus)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Startup,
    Playing,
    Paused,
}

// 11.864406779661017 chars per sec
#[derive(Debug, Default)]
struct LocalPathCurvature(Vec3);

// use std::cmp::Eq;
// use std::cmp::PartialEq;
// use std::hash::Hash;

// //#[derive(PartialEq, Eq, Hash)]
// //#[derive(PartialEq, Eq, Hash)]
// #[derive(PartialEq, Eq, Hash)]
// struct KeyTimestamp(KeyCode, f64);

// impl LocalPathCurvature {
//     fn get_new_average(&mut self, key_event: (KeyCode, f64)) -> f64 {
//         self.recent_keys.insert(key_event);
//         0.0
//     }
// }

// impl Default for LocalPathCurvature {
//     fn default() -> Self {
//         LocalPathCurvature {
//             curvature: Vec3::ZERO,
//         }
//     }
// }

fn window_focus(
    mut focus_events: EventReader<bevy::window::WindowFocused>,
    mut app_state: ResMut<State<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if *(app_state.current()) == AppState::Startup {
        for key in keys.get_pressed() {
            if *key == KeyCode::Space {
                app_state.overwrite_set(AppState::Playing).unwrap();
            }
        }
        return;
    }
    for ev in focus_events.iter() {
        if ev.focused {
            app_state.overwrite_set(AppState::Playing).unwrap();
        } else {
            app_state.overwrite_set(AppState::Paused).unwrap();
        }
    }
}

fn rocket_forward(mut camera_query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    //time.delta();
    let mut transform = camera_query.single_mut();
    let direction = transform.local_z();
    transform.translation -= direction * time.delta_seconds();
}

fn steer(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    mut curvature: ResMut<LocalPathCurvature>,
) {
    let gain = 0.1;
    let nudge = TAU / 10000.0;
    let mut left = 0.0;
    let mut up = 0.0;
    let mut had_input = false;
    for key in keys.get_pressed() {
        //let now = time.seconds_since_startup();
        match key {
            KeyCode::Left => {
                left += nudge * (curvature.0.x + 1.0);
                had_input = true;
                curvature.0.x += gain;
            }
            KeyCode::Right => {
                left -= nudge * (curvature.0.x + 1.0);
                had_input = true;
                curvature.0.x += gain;
            }
            KeyCode::Up => {
                up -= nudge * (curvature.0.y + 1.0);
                had_input = true;
                curvature.0.x += gain;
            }
            KeyCode::Down => {
                up += nudge * (curvature.0.y + 1.0);
                had_input = true;
                curvature.0.x += gain;
            }
            _ => (),
        }
    }
    if !had_input {
        if curvature.0.x > 0.0 {
            curvature.0.x -= gain;
            if curvature.0.x < 0.0 {
                curvature.0.x = 0.0;
            }
        }
        if curvature.0.y > 0.0 {
            curvature.0.y -= gain;
            if curvature.0.y < 0.0 {
                curvature.0.y = 0.0;
            }
        }
    }
    let mut transform = query.single_mut();
    if left != 0.0 || up != 0.0 {
        let local_x = transform.local_x();
        let local_y = transform.local_y();
        let local_z = transform.local_z();
        // Oh, I bet I need some math here.
        transform.rotate(Quat::from_axis_angle(local_x, up));
        transform.rotate(Quat::from_axis_angle(local_z, left));
        transform.rotate(Quat::from_axis_angle(local_y, left));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 60.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    for n in 0..10 {
        let mut side = 1.0;
        if n % 2 == 0 {
            side = -1.0;
        }
        let step = 2.0 * n as f32;
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(2.0 * side, 0.5, step),
            ..Default::default()
        });
    }
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.5, -1.0)
            .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        ..Default::default()
    });
}
