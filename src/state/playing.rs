use bevy::prelude::*;
use super::super::{
    resource::{
        config::Config,
        tilemap::Tilemap
    },
    component::*,
    enums::*,
};


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    commands.spawn((Character {
        sprite: SpriteBundle {
            texture: asset_server.load(&config.player.image_path),
            transform: Transform::from_xyz(0.0, 0.0, config.player.z_height),
            sprite: Sprite {
                custom_size: Some(Vec2::new(config.player.width, config.player.height)),
                ..default()
            },
            ..default()
        },
        velocity: Velocity::new(0.0, 0.0),
        },
        Player,
    ));

    commands.spawn((
        Character {
            sprite: SpriteBundle {
                texture: asset_server.load(&config.enemy.basic.image_path),
                transform: Transform::from_xyz(0.0, 0.0, config.enemy.basic.z_height),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(config.enemy.basic.width, config.enemy.basic.height)),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity::new(10.0, 0.0),
        },
        Enemy {
            move_behaviour: MoveBehaviour::PointedToPlayer,
        },
    ));
}

pub fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    tilemap: Res<Tilemap>,
) {
    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            let map_x: f32 = (x as f32 - tilemap.centre_x as f32) * config.map.tile_size as f32;
            let map_y: f32 = (y as f32 - tilemap.centre_y as f32) * config.map.tile_size as f32;

            commands.spawn((
                Tile,
                SpriteBundle {
                    texture: asset_server.load(&config.map.grass_texture_path),
                    transform: Transform::from_xyz(map_x,
                                                   map_y,
                                                   config.map.default_z_height),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(config.map.tile_size, config.map.tile_size)),
                        ..default()
                    },
                    ..default()
                }
            ));
        }
    }
}
