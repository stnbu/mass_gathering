use bevy::prelude::*;
use bevy_egui::{
    egui::{style::Margin, Color32, Frame, RichText, TopBottomPanel},
    EguiContext, EguiPlugin,
};
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use rand::Rng;
use std::f32::consts::TAU;
mod physics;

mod flying_transform;
use flying_transform as ft;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::MIDNIGHT_BLUE * 0.1))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_state(AppState::Startup)
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(ft::move_forward)
                .with_system(ft::steer)
                .with_system(physics::freefall)
                .with_system(physics::collision_events),
        )
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(handle_game_state)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(hud)
        .add_system(ft::update_relative_transforms)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum AppState {
    Startup,
    Playing,
    Paused,
    Menu,
}

fn handle_game_state(mut app_state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    use AppState::*;
    use KeyCode::*;
    let next_state =
        keys.get_just_pressed()
            .fold(None, |_state, key| match (*app_state.current(), *key) {
                (Playing, Space) => Some(Paused),
                (Paused, Space) => Some(Playing),
                (Menu, M) => Some(Playing),
                (_, M) => Some(Menu),
                (Startup, _) => Some(Playing),
                _ => None,
            });
    if let Some(state) = next_state {
        let _ = app_state.overwrite_set(state);
    }
}

// Take the latitude (poles are [1,-1]) and the longitude (portion around, starting at (0,0,1))
// and return the x, y, z on the unit sphere.
fn latlon_to_cartesian(lat: f32, lon: f32) -> Vec3 {
    let theta = (lat * 2.0 - 1.0).acos(); // latitude. -1 & 1 are poles. 0 is equator.
    let phi = lon * TAU; // portion around the planet `[0,1)` (from Greenwich)
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();
    Vec3::new(x, y, z)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec3::ZERO;

    let mut rng = rand::thread_rng();
    let mut rf = || rng.gen::<f32>();
    let pair_count = 40;
    for _ in 0..pair_count {
        let position = latlon_to_cartesian(rf(), rf()) * (rf() * 40.0 + 10.0);
        let velocity = latlon_to_cartesian(rf(), rf()) * Vec3::new(10.0, rf() * 0.1, 10.0);
        let radius = rf() + 2.0;
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
    commands
        .spawn_bundle(Camera3dBundle {
            transform: ft::FlyingTransform::from_xyz(0.0, 200.0, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..Default::default()
        })
        .insert(ft::Movement::default());

    commands
        .spawn_bundle(PointLightBundle {
            transform: Transform::default(),
            point_light: PointLight {
                intensity: 5000.0,
                range: 1000.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ft::RelativeTransform(Transform::from_xyz(0.0, 0.0, -25.0)));
    commands
        .spawn_bundle(PointLightBundle {
            transform: Transform::default(),
            point_light: PointLight {
                intensity: 1000000.0,
                range: 1000.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ft::RelativeTransform(Transform::from_xyz(0.0, 0.0, 100.0)));
}

fn hud(mut ctx: ResMut<EguiContext>, query: Query<(&ft::Movement, &Transform)>) {
    let (movement, transform) = query.get_single().unwrap();
    TopBottomPanel::top("hud")
        .frame(Frame {
            outer_margin: Margin::symmetric(10.0, 20.0),
            fill: Color32::TRANSPARENT,
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.label(RichText::new("Keys:").color(Color32::GREEN));
            ui.label(RichText::new("  Arrow Keys:\tPitch & Roll").color(Color32::GREEN));
            ui.label(RichText::new("  Z & X:\t\t\tYaw").color(Color32::GREEN));
            ui.label(RichText::new("  PgUp/PgDn:\tSpeed").color(Color32::GREEN));
            ui.label(RichText::new("\n"));
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
        });
}
