use serde::{Deserialize, Serialize};
use std::{ fs, fmt };
use log::{ info };
use bevy::prelude::*;
use super::super::enums::*;
use std::time::SystemTime;


pub const DEFAULT_CONFIG_FILE: &str = "config.ron";


#[derive(Resource, Debug)]
pub struct RoundInfo {
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
    pub number: u64,
    pub enemy_counts: EnemyCount,
}

#[derive(Resource, Debug)]
pub struct EnemyCount {
    pub Basic: u64,
    pub Tank: u64,
}


impl Default for RoundInfo {
    fn default() -> Self {
        RoundInfo {
            number: 0,
            enemy_counts: EnemyCount {
                Basic: 0,
                Tank: 0,
            },
            start_time: None,
            end_time: None,
        }
    }
}
