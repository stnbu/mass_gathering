use bevy::prelude::*;
use bevy_egui::{
    egui::{Color32, Frame, RichText, SidePanel},
    EguiContext, EguiPlugin,
};
use bevy_rapier3d::prelude::*;
use particular::prelude::*;
use rand::Rng;
use rapier3d::geometry::SharedShape;
use std::f32::consts::PI;

mod bodies;
mod space_camera;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(ParticleSet::<bodies::Body>::new())
        .add_state(AppState::Startup)
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(space_camera::move_forward)
                .with_system(space_camera::steer)
                .with_system(bodies::update_particles),
        )
        .insert_resource(space_camera::CameraConfig {
            transform: Transform::from_translation(Vec3::new(100.0, 100.0, 100.0))
                .looking_at(Vec3::new(1.0, 1.0, 1.0), Vec3::Y),
        })
        .add_plugin(space_camera::SpaceCamera)
        .add_startup_system(setup)
        // "for prototyping" -- unclean shutdown, havoc under wasm.
        .add_system(bevy::window::close_on_esc)
        .add_system(handle_game_state)
        // -- Uncomment once we have collision working:
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_system_to_stage(CoreStage::PostUpdate, display_events)
        .add_system(hud)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Startup,
    Playing,
    Paused,
}

fn _display_events(mut events: EventReader<CollisionEvent>) {
    for _collision_event in events.iter() {
        //
    }
}

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
    } else {
        if *(app_state.current()) == AppState::Startup {
            app_state.overwrite_set(AppState::Playing).unwrap();
        } else {
            if let Some(new_state) = toggle_pause(app_state.current()) {
                app_state.overwrite_set(new_state).unwrap();
            }
        }
    }
}

#[derive(Bundle)]
struct Planet {
    #[bundle]
    pbr: PbrBundle,
    point_mass: bodies::PointMass,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut particle_set: ResMut<ParticleSet<bodies::Body>>,
) {
    let mut rng = rand::thread_rng();
    let mut rf = || rng.gen::<f32>();
    for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                let x = ((x - 2) * 4) as f32 + rf();
                let y = ((y - 2) * 4) as f32 + rf();
                let z = ((z - 2) * 4) as f32 + rf();
                let position = Vec3::new(x, y, z);
                let r = rf();
                let g = rf();
                let b = rf();
                let radius = rf() + 1.0;
                let pbr = PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius,
                        ..Default::default()
                    })),
                    material: materials.add(Color::rgb(r, g, b).into()),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                };
                let entity = commands
                    .spawn_bundle(Planet {
                        pbr,
                        point_mass: bodies::PointMass {},
                    })
                    .insert(RigidBody::Dynamic)
                    .insert(<SharedShape as Into<Collider>>::into(SharedShape::ball(
                        radius,
                    )))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .id();
                let mass = 0.75 * PI * radius.powf(3.0);
                let velocity = Vec3::new(rf(), rf(), rf());
                particle_set.add_massive(bodies::Body::new(position, mass, velocity, entity));
            }
        }
    }
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1600000.0 * 0.8,
            range: 1000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(220.0, 200.0, 45.0),
        ..Default::default()
    });
}

fn hud(mut ctx: ResMut<EguiContext>, query: Query<&space_camera::Movement>) {
    let movement = query.get_single().unwrap();
    SidePanel::left("hud")
        .frame(Frame {
            fill: Color32::TRANSPARENT,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.label(RichText::new(format!("Speed: {}", movement.speed)).color(Color32::GREEN));
            ui.separator();
            ui.label(RichText::new("Arrow Keys: Pitch & Roll").color(Color32::GREEN));
            ui.label(RichText::new("Z & X: Yaw").color(Color32::GREEN));
            ui.label(RichText::new("PgUp/PgDn: Speed").color(Color32::GREEN));
        });
}
