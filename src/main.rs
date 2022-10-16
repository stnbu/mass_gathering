use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system_to_stage(CoreStage::PostUpdate, display_events)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, -50.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

fn display_events(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }
}

pub fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(Collider::cuboid(3.0, 3.0, 3.0))
        .insert(ColliderMassProperties::Density(1.0));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 1.0,
                ..Default::default()
            })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(ActiveEvents::COLLISION_EVENTS);
}
