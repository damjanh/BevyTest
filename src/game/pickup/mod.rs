use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::game::SimulationState;
use crate::AppState;

pub const PICKUP_SIZE: f32 = 32.0;
pub const NUM_OF_PICKUPS: i8 = 3;

pub struct PickupPlugin;
impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PickupSpawnTimer>()
            .add_system(spawn_pickups.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (tick_pickup_spawn_timer, spawn_pickups_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_pickups.in_schedule(OnExit(AppState::Game)));
    }
}
