use bevy::prelude::*;
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
		.with_system(steer)
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
#[derive(Debug)]
struct LocalPathCurvature {
    curvature: Vec3,
    pub recent_keys: Vec<(KeyCode, f64)>,
}

impl Default for LocalPathCurvature {
    fn default() -> Self {
        LocalPathCurvature {
            curvature: Vec3::ZERO,
	    recent_keys: Vec::new(),
        }
    }
}

fn window_focus(
    mut focus_events: EventReader<bevy::window::WindowFocused>,
    mut app_state: ResMut<State<AppState>>,
    keys: Res<Input<KeyCode>>
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
    let nudge = TAU / 10000.0;
    let mut roll = 0.0;
    let mut up = 0.0;
    for key in keys.get_pressed() {
	let now = time.seconds_since_startup();
        match key {
            KeyCode::Left => {
		roll += nudge;
		curvature.recent_keys.push((*key, now));
	    },
            KeyCode::Right => roll -= nudge,
            KeyCode::Up => up -= nudge,
            KeyCode::Down => {
		eprintln!("{:?}", curvature);
		up += nudge;},
            _ => (),
        }
    }
    let mut transform = query.single_mut();
    if roll != 0.0 || up != 0.0 {
        let local_x = transform.local_x();
        let local_y = transform.local_y();
        let local_z = transform.local_z();
        // Oh, I bet I need some math here.
        transform.rotate(Quat::from_axis_angle(local_x, up));
        transform.rotate(Quat::from_axis_angle(local_z, roll));
        transform.rotate(Quat::from_axis_angle(local_y, roll));
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
