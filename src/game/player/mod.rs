use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::game::SimulationState;
use crate::AppState;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (player_movement, confine_player_movement)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running))
                    .chain(),
            )
            .add_systems(
                (enemy_player_collision, pickup_player_collision)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
