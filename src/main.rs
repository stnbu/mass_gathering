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
    egui::{style::Margin, Color32, Frame, RichText, TopBottomPanel},
    EguiContext, EguiPlugin,
};
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use rand::Rng;

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    //global_config: Res<gf::GlobalConfig>,
) {
    use std::f32::consts::TAU;
    rapier_config.gravity = Vec3::ZERO;

    let mut rng = rand::thread_rng();
    let mut rf = || rng.gen::<f32>();
    let pair_count = 40;
    for _ in 0..pair_count {
        //let direction = Vec3::new(rf(), rf(), rf());
        let phi = rf() * TAU;
        let theta = (rf() * 2.0 - 1.0).acos();
        let x = theta.sin() * phi.cos();
        let y = theta.sin() * phi.sin();
        let z = theta.cos();
        let direction = Vec3::new(x, y, z).normalize();

        let position = (direction * 50.0) + 4.0;
        let perturbence = (position.length() * 0.1) * Vec3::new(rf(), rf(), rf());
        let velocity = (position + perturbence) * 0.2;
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
    commands
        .spawn_bundle(Camera3dBundle {
            transform: ft::FlyingTransform::from_xyz(0.0, 0.0, -200.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
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

/*
phi = random(0.0, TAU)
theta = arccos(random(-1.0, 1.0))
x = sin(theta) * cos(phi)
y = sin(theta) * sin(phi)
z = cos(theta)
*/

/*
phi = r() * TAU;
theta = arccos(r() * 2.0 - 1.0);
x = sin(theta) * cos(phi);
y = sin(theta) * sin(phi);
z = cos(theta);
*/

/*
phi = r() * TAU;
theta = (r() * 2.0 - 1.0).acos();
x = theta.sin() * phi.cos();
y = theta.sin() * phi.sin();
z = theta.cos();
*/
