use bevy::{
    prelude::*,
};
use log::warn;
use super::super::{
    component::*,
    enums::*,
    event::*,
    resource::{
        tilemap::Tilemap,
        config::{
            Config,
            EnemyConfig,
        },
        round::RoundInfo,
    },
};
use rand::Rng;
use std::time::Duration;


pub fn enemy_spawner(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEnemyEvent>,
    config: Res<Config>,
    asset_server: Res<AssetServer>,
    tilemap: Res<Tilemap>,
) {
    for event in spawn_events.iter() {
        info!("Spawning a {:?} enemy", event.enemy_type);

        let enemy_config: &EnemyConfig = match event.enemy_type {
            EnemyType::Basic => &config.enemy.basic
        };

        let transform: Transform = match event.location {
            Some((x, y)) => Transform::from_xyz(x, y, enemy_config.z_height),
            None => {
                let (mut x, mut y) = random_outside_map(tilemap.width, tilemap.height, tilemap.centre_x, tilemap.centre_y);
                (x, y) = tilemap.screen_pos_from_world_pos(x, y, config.map.tile_size);
                Transform::from_xyz(x, y, enemy_config.z_height)
            }
        };

        commands.spawn((
            Character {
                sprite: SpriteBundle {
                    texture: asset_server.load(&enemy_config.image_path),
                    transform: transform,
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

fn random_outside_map(x: usize, y: usize, centre_x: u64, centre_y: u64) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let left_right: f32 = rng.gen::<f32>();
    let up_down: f32 = rng.gen::<f32>();

    let enemy_x: f32;
    if left_right < 0.5 {
        enemy_x = -(x as f32 - centre_x as f32);
    } else {
        enemy_x = x as f32 - centre_x as f32;
    }

    let enemy_y: f32;
    if up_down < 0.5 {
        enemy_y = -(y as f32 - centre_y as f32);
    } else {
        enemy_y = y as f32 - centre_y as f32;
    }

    return (enemy_x, enemy_y);
}


pub fn enemy_caller(
    mut spawn_events: EventWriter<SpawnEnemyEvent>,
    mut round: ResMut<RoundInfo>,
    config: Res<Config>,
) {
    if round.enemy_counts.Basic > 0 {
        info!("Calling for spawner to make an enemy.");
        spawn_events.send(SpawnEnemyEvent{
            enemy_type: EnemyType::Basic,
            location: None,
        });
        round.enemy_counts.Basic = round.enemy_counts.Basic - 1;
    }
}
