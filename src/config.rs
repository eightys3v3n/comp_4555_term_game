use serde::{Deserialize, Serialize};
use std::fs;
use log::{info, warn};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub player: PlayerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerConfig {
    pub image_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { player: PlayerConfig::default() }
    }
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self { image_path: String::from("unconfigured.png") }
    }
}

pub fn load_config(file_path: &str) -> Config {
    let config_str: &str = &fs::read_to_string(file_path)
        .expect("Wasn't able to read config.ron");

    let conf = match ron::from_str(config_str) {
        Ok(v) => {
            info!("Loaded config file");
            v
        },
        Err(err) => {
            warn!("Failed to load config file, using default conig");
            Config::default()
        }
    };

    info!("Loaded config: {}", conf.player.image_path);

    return conf;
}
