use bevy::prelude::*;

pub const PICKUP_SPAWN_TIMEOUT_SECONDS: f32 = 2.0;
pub const ENEMY_SPAWN_TIMEOUT_SECONDS: f32 = 5.0;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}
impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

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

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIMEOUT_SECONDS, TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}
impl Default for HighScores {
    fn default() -> Self {
        Self { scores: Vec::new() }
    }
}
