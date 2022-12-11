use serde::{Deserialize, Serialize};
use std::{ fs, fmt };
use log::{ info };
use bevy::prelude::*;
use super::super::enums::MoveBehaviour;


pub const DEFAULT_CONFIG_FILE: &str = "config.ron";


#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct Config {
    pub player: PlayerConfig,
    pub map: MapConfig,
    pub enemy: EnemyTypesConfig,
    pub performance: PerformanceConfig,
    pub menu: MenuConfig,
    pub window: WindowConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PerformanceConfig {
    pub enemy_movement_frequency: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapConfig {
    pub grass_texture_path: String,
    pub default_z_height: f32,
    pub tile_size: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerConfig {
    pub image_path: String,
    pub move_speed: f32,
    pub width: f32,
    pub height: f32,
    pub z_height: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnemyTypesConfig {
    pub basic: EnemyConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnemyConfig {
    pub image_path: String,
    pub move_speed: f32,
    pub width: f32,
    pub height: f32,
    pub move_behaviour: MoveBehaviour,
    pub z_height: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MenuConfig {
    pub r#continue: ButtonConfig,
    pub new_game: ButtonConfig,
    pub exit: ButtonConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ButtonConfig {
    pub image_path: String,
    pub z_height: f32, // should be above the entire playable world.
    pub width: f32,
    pub height: f32,
}



impl Default for Config {
    fn default() -> Self {
        load_config()
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "
config:
  window:
    width: {}
    height: {}
  player:
    image_path:{}
    move_speed:{}
    width:{}
    height:{}
  map:
    grass_texture_path:{}
    default_z_height:{}
    tile_size:{}
  enemy:
    basic:
      move_speed:{}
  performance:
    enemy_movement_frequency: {}
  menu:
    continue:
      button_image: {}
      z_height: {}
      height: {}
      width: {}
    new_game:
      button_image: {}
      z_height: {}
      height: {}
      width: {}
    exit:
      button_image: {}
      z_height: {}
      height: {}
      width: {}",
            self.window.width,
            self.window.height,
            self.player.image_path,
            self.player.move_speed,
            self.player.width,
            self.player.height,
            self.map.grass_texture_path,
            self.map.default_z_height,
            self.map.tile_size,
            self.enemy.basic.move_speed,
            self.performance.enemy_movement_frequency,
            self.menu.r#continue.image_path,
            self.menu.r#continue.z_height,
            self.menu.r#continue.height,
            self.menu.r#continue.width,
            self.menu.new_game.image_path,
            self.menu.new_game.z_height,
            self.menu.new_game.height,
            self.menu.new_game.width,
            self.menu.exit.image_path,
            self.menu.exit.z_height,
            self.menu.exit.height,
            self.menu.exit.width,
       )
    }
}


pub fn load_config() -> Config {
    let config_str: &str = &fs::read_to_string(DEFAULT_CONFIG_FILE)
        .expect("Wasn't able to read config.ron");

    let conf = match ron::from_str::<Config>(config_str) {
        Ok(v) => {
            info!("Loaded config file");
            v
        },
        Err(err) => {
            panic!("Failed to load config file: {}", err)
        }
    };

    info!("{}", conf);

    return conf;
}
