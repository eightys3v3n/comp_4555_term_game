use serde::{Deserialize, Serialize};
use std::{ fs, fmt };
use log::{ info, warn };
use bevy::prelude::*;


pub const DEFAULT_CONFIG_FILE: &str = "config.ron";


#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct Config {
    pub player: PlayerConfig,
    pub map: MapConfig,
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
    pub default_z_height: f32,
}

impl Default for Config {
    fn default() -> Self {
        load_config()
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\
config:\n\
  player:\n\
    image_path:{}\n\
    move_speed:{}\n\
    width:{}\n\
    height:{}\n\
  map:\n\
    grass_texture_path:{}\n\
    default_z_height:{}\n\
    tile_size:{}\n",
            self.player.image_path,
            self.player.move_speed,
            self.player.width,
            self.player.height,
            self.map.grass_texture_path,
            self.map.default_z_height,
            self.map.tile_size
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
