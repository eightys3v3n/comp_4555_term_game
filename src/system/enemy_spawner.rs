use bevy::{
    prelude::*,
};
use log::warn;
use super::super::{
    component::*,
    enums::EnemyType,
    event::*,
    resource::config::{
        Config,
        EnemyConfig,
    },
};

pub fn enemy_spawner(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEnemyEvent>,
    config: Res<Config>,
    asset_server: Res<AssetServer>,
) {
    for event in spawn_events.iter() {
        info!("Spawning a {:?} enemy", event.enemy_type);

        let enemy_config: &EnemyConfig = match event.enemy_type {
            EnemyType::Basic => &config.enemy.basic
        };

        commands.spawn((
            Character {
                sprite: SpriteBundle {
                    texture: asset_server.load(&enemy_config.image_path),
                    transform: Transform::from_xyz(0.0, 0.0, enemy_config.z_height),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(enemy_config.width, enemy_config.height)),
                        ..default()
                    },
                    ..default()
                },
                velocity: Velocity::new(10.0, 0.0),
                health: Health::new(config.player.default_health),
            },
            Enemy {
                move_behaviour: enemy_config.move_behaviour.clone(),
            },
        ));
    }
}
