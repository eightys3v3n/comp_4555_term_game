use super::{
    resource::config::EnemyConfig,
    enums::EnemyType,
};
use bevy::{
    prelude::*,
};


pub struct SpawnEnemyEvent {
    pub enemy_type: EnemyType,
    pub location: Option<(f32, f32)>,
}

pub struct RoundEndEvent {
    pub round_number: u64,
}

pub struct RoundStartEvent {
    pub round_number: u64,
}
