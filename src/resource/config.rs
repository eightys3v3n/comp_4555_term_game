use serde::{Deserialize, Serialize};
use std::fs;
use log::{info, warn};
use bevy::prelude::*;


pub const DEFAULT_CONFIG_FILE: &str = "config.ron";


#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct Config {
    pub player: PlayerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerConfig {
    pub image_path: String,
    pub move_speed: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Config {
    fn default() -> Self {
        load_config()
    }
}


pub fn load_config() -> Config {
    let config_str: &str = &fs::read_to_string(DEFAULT_CONFIG_FILE)
        .expect("Wasn't able to read config.ron");

    let conf = match ron::from_str::<Config>(config_str) {
        Ok(v) => {
            info!("Loaded config file");
            info!("Player Image: {}", v.player.image_path);
            v
        },
        Err(err) => {
            warn!("Failed to load config file, using default config. {}", err);
            Config::default()
        }
    };

    info!("Loaded config: {}", conf.player.image_path);

    return conf;
}
