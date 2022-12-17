use serde::{Deserialize, Serialize};
use std::{ fs, fmt };
use log::{ info };
use bevy::prelude::*;
use super::super::enums::*;


pub const DEFAULT_CONFIG_FILE: &str = "config.ron";


#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct Config {
    pub player: PlayerConfig,
    pub map: MapConfig,
    pub enemy: EnemyTypesConfig,
    pub performance: PerformanceConfig,
    pub menu: MenuConfig,
    pub window: WindowConfig,
    pub round: RoundConfig,
    pub bullet: BulletsConfig,
    pub sound: SoundConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SoundConfig {
    pub shoot_basic_path: String,
    pub background_music_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulletsConfig {
    pub basic: BulletConfig,
    pub z_height: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BulletConfig {
    pub image_path: String,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub size: f32,
    pub damage: f32,
    pub range: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub round_counter_text: String,
    pub enemies_counter_text: String,
    pub points_counter_text: String,
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
    pub default_health: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnemyTypesConfig {
    pub basic: EnemyConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnemyConfig {
    pub r#type: EnemyType,
    pub image_path: String,
    pub move_speed: f32,
    pub width: f32,
    pub height: f32,
    pub move_behaviour: Vec<MoveBehaviour>,
    pub z_height: f32,
    pub damage: f32,
    pub hit_cooldown: u64,
    pub health: f32,
    pub points: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MenuConfig {
    pub button_font: String,
    pub new_game: ButtonConfig,
    pub exit: ButtonConfig,
    pub main_menu: ButtonConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ButtonConfig {
    pub text: String,
    pub image_path: String,
    pub z_height: f32, // should be above the entire playable world.
    pub width: f32,
    pub height: f32,
    pub id: ButtonID,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoundConfig {
    pub basic_multiplier: f32,
    pub start_delay: u64,
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
    default_health:{}
  map:
    grass_texture_path:{}
    default_z_height:{}
    tile_size:{}
  enemy:
    {:?}:
      move_speed: {}
      move_behaviour: {:?}
  performance:
    enemy_movement_frequency: {}
  round:
    basic_multiplier: {}
    round_delay: {}
  menu:
    button_font: {}
    new_game:
      id: {:?}
      text: {}
      image: {}
      z_height: {}
      height: {}
      width: {}
    exit:
      id: {:?}
      text: {}
      image: {}
      z_height: {}
      height: {}
      width: {}
    main_menu:
      id: {:?}
      text: {}
      image: {}
      z_height: {}
      height: {}
      width: {}",
            self.window.width,
            self.window.height,
            self.player.image_path,
            self.player.move_speed,
            self.player.width,
            self.player.height,
            self.player.default_health,
            self.map.grass_texture_path,
            self.map.default_z_height,
            self.map.tile_size,
            self.enemy.basic.r#type,
            self.enemy.basic.move_speed,
            self.enemy.basic.move_behaviour,
            self.performance.enemy_movement_frequency,
            self.round.basic_multiplier,
            self.round.start_delay,
            self.menu.button_font,
            self.menu.new_game.id,
            self.menu.new_game.text,
            self.menu.new_game.image_path,
            self.menu.new_game.z_height,
            self.menu.new_game.height,
            self.menu.new_game.width,
            self.menu.exit.id,
            self.menu.exit.text,
            self.menu.exit.image_path,
            self.menu.exit.z_height,
            self.menu.exit.height,
            self.menu.exit.width,
            self.menu.main_menu.id,
            self.menu.main_menu.text,
            self.menu.main_menu.image_path,
            self.menu.main_menu.z_height,
            self.menu.main_menu.height,
            self.menu.main_menu.width,
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
