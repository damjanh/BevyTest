use bevy::prelude::*;

mod pickup;
mod player;
mod systems;

pub mod enemy;
pub mod score;

use crate::events::GameOver;
use crate::AppState;

use enemy::EnemyPlugin;
use pickup::PickupPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use systems::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(PickupPlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
