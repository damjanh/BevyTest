use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_systems((player_movement, confine_player_movement).chain())
            .add_system(enemy_player_collision)
            .add_system(pickup_player_collision);
    }
}
