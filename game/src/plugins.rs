use crate::*;
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};

#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::ToServer>();
        app.add_event::<events::ToClient>();
        app.add_state(resources::GameState::Stopped);
        app.add_event::<physics::MassCollisionEvent>();
        app.add_event::<physics::DespawnMassEvent>();
        app.init_resource::<resources::GameConfig>();
    }
}

#[derive(Default)]
pub struct SimulationPlugin;

// FIXME: The mass-mass collision and merging code has been disabled. This
// code seems to cause a lot of problems/confusion. Let us let things settle
// a bit first, then we'll tackle that whole mess.
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
                .with_system(physics::freefall),
        );
    }
}
