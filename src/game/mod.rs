use bevy::prelude::*;

mod pickup;
mod player;

pub mod enemy;
pub mod score;

use crate::events::GameOver;

use enemy::EnemyPlugin;
use pickup::PickupPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(PickupPlugin);
    }
}
