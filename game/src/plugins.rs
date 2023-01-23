use crate::*;

#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::MassIDToEntity::default());
        app.add_event::<events::ToServer>();
        app.add_event::<events::ToClient>();
        app.add_state(resources::GameState::Stopped);
        app.init_resource::<physics::PhysicsConfig>();
        app.add_event::<physics::MassCollisionEvent>();
        app.add_event::<physics::DespawnMassEvent>();
        app.insert_resource(resources::Lobby::default());
    }
}

#[derive(Default)]
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            ..Default::default()
        });
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_system_set(
            SystemSet::on_update(resources::GameState::Running)
                .with_run_criteria(with_gravity)
                .with_system(physics::handle_despawn_mass)
                // .with_system(physics::handle_mass_collisions.before(physics::handle_despawn_mass))
                // .with_system(physics::merge_masses.before(physics::handle_despawn_mass))
                .with_system(physics::freefall.before(physics::handle_despawn_mass)),
        );
    }
}
