use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub const PICKUP_SIZE: f32 = 32.0;
pub const NUM_OF_PICKUPS: i8 = 3;

pub struct PickupPlugin;
impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PickupSpawnTimer>()
            .add_startup_system(spawn_pickups)
            .add_system(tick_pickup_spawn_timer)
            .add_system(spawn_pickups_over_time);
    }
}
