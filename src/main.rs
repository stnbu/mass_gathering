use bevy::prelude::*;
use std::f32::consts::TAU;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Curvature::default())
        .add_state(AppState::Startup)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(rocket_forward)
                .with_system(steer),
        )
        .add_system(bevy::window::close_on_esc) // "or prototyping" -- unclean shutdown
        .add_system(handle_game_state)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Startup,
    Playing,
    Paused,
}

#[derive(Debug, Default)]
struct Curvature(Vec3);

fn toggle_pause(current: &AppState) -> Option<AppState> {
    match current {
        AppState::Paused => Some(AppState::Playing),
        AppState::Playing => Some(AppState::Paused),
        _ => None,
    }
}

fn handle_game_state(
    mut focus_events: EventReader<bevy::window::WindowFocused>,
    mut app_state: ResMut<State<AppState>>,
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    let mut poked = false; // space bar hit or window left-clicked
    for key in keys.get_just_pressed() {
        if *key == KeyCode::Space {
            poked = !poked;
        }
    }
    if mouse_buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        poked = !poked;
    }

    if !poked {
        for ev in focus_events.iter() {
            if ev.focused {
                app_state.overwrite_set(AppState::Playing).unwrap();
            } else {
                app_state.overwrite_set(AppState::Paused).unwrap();
            }
        }
        return;
    }

    if *(app_state.current()) == AppState::Startup {
        app_state.overwrite_set(AppState::Playing).unwrap();
    } else {
        if let Some(new_state) = toggle_pause(app_state.current()) {
            app_state.overwrite_set(new_state).unwrap();
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
    mut curvature: ResMut<Curvature>,
) {
    let gain = 0.2;
    let nudge = TAU / 10000.0;
    let mut roll = 0.0;
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let mut had_input = false;
    for key in keys.get_pressed() {
        match key {
            KeyCode::Left => {
                roll += nudge * (curvature.0.z + 1.0);
                had_input = true;
                curvature.0.z += gain;
            }
            KeyCode::Right => {
                roll -= nudge * (curvature.0.z + 1.0);
                had_input = true;
                curvature.0.z += gain;
            }
            KeyCode::Up => {
                pitch -= nudge * (curvature.0.x + 1.0);
                had_input = true;
                curvature.0.x += gain;
            }
            KeyCode::Down => {
                pitch += nudge * (curvature.0.x + 1.0);
                had_input = true;
                curvature.0.x += gain;
            }
            KeyCode::Z => {
                yaw += nudge * (curvature.0.y + 1.0);
                had_input = true;
                curvature.0.y += gain;
            }
            KeyCode::X => {
                yaw -= nudge * (curvature.0.y + 1.0);
                had_input = true;
                curvature.0.y += gain;
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
        if curvature.0.z > 0.0 {
            curvature.0.z -= gain;
            if curvature.0.z < 0.0 {
                curvature.0.z = 0.0;
            }
        }
    }
    let mut transform = query.single_mut();
    if roll != 0.0 || pitch != 0.0 || yaw != 0.0 {
        let local_x = transform.local_x();
        let local_y = transform.local_y();
        let local_z = transform.local_z();
        // Oh, I bet I need some math here.
        transform.rotate(Quat::from_axis_angle(local_x, pitch));
        transform.rotate(Quat::from_axis_angle(local_z, roll));
        transform.rotate(Quat::from_axis_angle(local_y, yaw));
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
