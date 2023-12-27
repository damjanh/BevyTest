mod pickup;
mod player;
mod systems;

pub mod enemy;
pub mod events;
pub mod score;

use events::*;
use systems::*;

use enemy::EnemyPlugin;
use pickup::PickupPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PickupPlugin)
        .add_plugin(ScorePlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
