use super::{
    resource::config::EnemyConfig,
    enums::*,
};
use bevy::{
    prelude::*,
};


pub struct FireBulletEvent {
    pub bullet_type: BulletType,
    pub start_transform: Transform,
}

pub struct CollideEvent {
    pub from_entity_type: EntityType,
    pub from_entity_id: Entity,
    pub to_entity_id: Entity,
    pub to_entity_type: EntityType,
}

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
