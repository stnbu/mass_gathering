/// FIXME
///
/// How Art Me Broken --
///
/// Before I forget, some longform about a recent new and terrible bug:
/// Particular worked fine, needed to merge planets so dropped it and now
/// use a `Query` as my particle set. Math should be same but gravity way
/// stronger now. So naturally I just mul'd it by a tiny number. That fixes
/// things sorta. I'm not sure it takes us back to where we were but it
/// _looks_ like it does. But then I turn on the collision stuff.
///
/// The "merging" stuff is weird and messy. Reasonably we will see entities
/// that we just despawned because of 3-way collisions in the queue, etc.
/// I chose just to skip those collisions. That seems to solve problems to
/// the point where all is as expected, except when we get to the last few
/// planets, the merging accelerates and BWOOP all are one gigantic planet.
/// (and that planet seems too big...) I'm not sure if this is a result of
/// the initial "cloud" not having enough outward momentum or ...what.
///
/// The Final Planet being so large makes me wonder if our merging math is
/// wrong. Probably.
use bevy::prelude::*;
use bevy_egui::{
    egui::{style::Margin, Color32, Frame, RichText, SidePanel, Slider},
    EguiContext, EguiPlugin,
};
use bevy_rapier3d::{
    parry::query::visitors::CompositePointContainmentTest,
    prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
};
use rand::Rng;

mod flying_transform;
mod physics;
use flying_transform as ft;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .insert_resource(GlobalConfig::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_state(AppState::Startup)
        .add_system(on_global_changes)
        .add_system(follow)
        .add_system(ft::steer)
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(ft::move_forward)
                .with_system(ft::steer)
                .with_system(physics::freefall)
                .with_system(physics::collision_events),
        )
        .add_startup_system(setup)
        // "for prototyping" -- unclean shutdown, havoc under wasm.
        .add_system(bevy::window::close_on_esc)
        .add_system(handle_game_state)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(hud)
        .run();
}

#[derive(Component)]
struct RelativeTransform {
    entity: Entity,
    transform: Transform,
}

fn follow(
    mut followers: Query<(&mut Transform, &RelativeTransform)>,
    reference_frames: Query<&Transform, Without<RelativeTransform>>,
) {
    for (mut follower, relative_transform) in followers.iter_mut() {
        if let Ok(frame) = reference_frames.get(relative_transform.entity) {
            *follower = frame.mul_transform(relative_transform.transform);
        }
    }
}

#[derive(Component, Default)]
struct GlobalConfigSubscriber;

fn on_global_changes(
    global_config: Res<GlobalConfig>,
    mut query: Query<
        (
            &mut RelativeTransform,
            Option<(&mut PointLight, &LightIndex)>,
        ),
        With<GlobalConfigSubscriber>,
    >,
    camera_query: Query<&Transform, (With<Camera>, Without<GlobalConfigSubscriber>)>,
) {
    if global_config.is_changed() {
        for (mut transform, light_opt) in query.iter_mut() {
            if let Some((mut light, index)) = light_opt {
                if let Ok(camera) = camera_query.get_single() {
                    if let Some(config) = global_config.lights.get(index.0) {
                        transform.transform.translation = (*config).position + camera.translation;
                        light.intensity = (*config).brightness;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Startup,
    Playing,
    Paused,
}

fn toggle_pause(current: &AppState) -> Option<AppState> {
    match current {
        AppState::Paused => Some(AppState::Playing),
        AppState::Playing => Some(AppState::Paused),
        _ => None,
    }
}

fn handle_game_state(
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

    if poked {
        if *(app_state.current()) == AppState::Startup {
            app_state.overwrite_set(AppState::Playing).unwrap();
        } else {
            if let Some(new_state) = toggle_pause(app_state.current()) {
                app_state.overwrite_set(new_state).unwrap();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    global_config: Res<GlobalConfig>,
) {
    rapier_config.gravity = Vec3::ZERO;

    let mut rng = rand::thread_rng();
    let mut rf = || rng.gen::<f32>();
    let pair_count = 40;
    for _ in 0..pair_count {
        let direction = Vec3::new(rf(), rf(), rf());
        let position = (direction * 60.0) + 10.0;
        let perturbence = (position.length() * 0.1) * Vec3::new(rf(), rf(), rf());
        let velocity = (position + perturbence) * 0.1;
        let radius = rf() + 1.0;
        for side in [-1.0, 1.0] {
            let color = Color::rgb(rf(), rf(), rf());
            physics::spawn_planet(
                radius,
                position * side,
                velocity * side,
                color,
                &mut commands,
                &mut meshes,
                &mut materials,
            );
        }
    }
    let cam = commands
        .spawn_bundle(Camera3dBundle {
            transform: ft::FlyingTransform::default(),
            ..Default::default()
        })
        .insert(ft::Movement::default())
        .id();
    for num in 0..global_config.lights.len() {
        commands
            .spawn_bundle(PointLightBundle {
                transform: Transform::default(),
                point_light: PointLight {
                    intensity: 0.0,
                    range: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(LightIndex(num))
            .insert(GlobalConfigSubscriber {})
            .insert(RelativeTransform {
                entity: cam,
                transform: Transform::default(),
            });
    }
}
#[derive(Component)]
struct LightIndex(usize);

struct LightConfig {
    position: Vec3,
    brightness: f32,
}

struct GlobalConfig {
    lights: Vec<LightConfig>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            lights: (0..5)
                .map(|_| LightConfig {
                    position: Vec3::ZERO,
                    brightness: 0.0,
                })
                .collect(),
        }
    }
}

fn hud(
    mut ctx: ResMut<EguiContext>,
    query: Query<(&ft::Movement, &Transform)>,
    mut global_config: ResMut<GlobalConfig>,
) {
    //, Slider, Stroke, Sense
    let (movement, transform) = query.get_single().unwrap();
    SidePanel::left("hud")
        .frame(Frame {
            outer_margin: Margin::symmetric(10.0, 20.0),
            fill: Color32::TRANSPARENT,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.separator();
            ui.label(RichText::new("Keys:").color(Color32::GREEN));
            ui.label(RichText::new("  Arrow Keys:\tPitch & Roll").color(Color32::GREEN));
            ui.label(RichText::new("  Z & X:\tYaw").color(Color32::GREEN));
            ui.label(RichText::new("  PgUp/PgDn:\tSpeed").color(Color32::GREEN));
            ui.separator();
            ui.label(
                RichText::new(format!("Your Speed: {}", movement.speed)).color(Color32::GREEN),
            );
            ui.label(
                RichText::new(format!(
                    "Your Location:\n  x: {}\n  y:{}\n  z:{}",
                    transform.translation.x, transform.translation.y, transform.translation.z
                ))
                .color(Color32::GREEN),
            );
            ui.separator();
            ui.separator();

            for (index, mut light) in global_config.lights.iter_mut().enumerate() {
                ui.label(RichText::new(format!("Light #{}", index)).color(Color32::GREEN));
                ui.add(Slider::new(&mut light.brightness, 0.0..=1280000.0).text("brightness"));
                ui.add(Slider::new(&mut light.position.x, -200.0..=200.0).text("x"));
                ui.add(Slider::new(&mut light.position.y, -200.0..=200.0).text("y"));
                ui.add(Slider::new(&mut light.position.z, -200.0..=200.0).text("z"));
                ui.separator();
            }
        });
}
