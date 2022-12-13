use super::{
    resource::config::EnemyConfig,
    enums::EnemyType,
};


pub struct SpawnEnemyEvent {
    pub enemy_type: EnemyType,
}
