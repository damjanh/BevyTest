use bevy::prelude::*;

pub const PICKUP_SPAWN_TIMEOUT_SECONDS: f32 = 2.0;

#[derive(Resource)]
pub struct PickupSpawnTimer {
    pub timer: Timer,
}
impl Default for PickupSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(PICKUP_SPAWN_TIMEOUT_SECONDS, TimerMode::Repeating),
        }
    }
}
