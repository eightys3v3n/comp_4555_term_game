use serde::{Deserialize, Serialize};
use std::fs;
use log::{info, warn};
use bevy::prelude::*;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub enum TileType {
    Grass,
}

#[derive(Resource, Default, Debug, Deserialize, Serialize)]
pub struct Tilemap {
    pub tiles: Vec<Tile>,
    pub width: u64,
    pub height: u64,
}

#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct Tile {
    r#type: TileType,
}

impl Tilemap {
    pub fn new(width: u64, height: u64) -> Self {
        let mut tiles: Vec<Tile> = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                tiles.push(Tile::new(TileType::Grass));
            }
        }

        Self {
            tiles: tiles,
            width: width,
            height: height,
        }
    }

    pub fn default() -> Self {
        load_map(String::from("world_1"))
    }

    pub fn get_tile() {

    }

    pub fn get_tile_mut() {

    }
}

impl Tile {
    pub fn new(r#type: TileType) -> Self {
        Self { r#type: r#type }
    }

    pub fn default() -> Self {
        Self{ r#type: TileType::Grass }
    }
}

pub fn load_map(name: String) -> Tilemap {
    let map_path: String = format!("{}.ron", name);

    let map_str: &str = &fs::read_to_string(map_path)
        .expect("Wasn't able to read the map file");

    let map = match ron::from_str::<Tilemap>(map_str) {
        Ok(v) => {
            info!("Loaded map file");
            info!("Map Size: {}x{}", v.width, v.height);
            v
        },
        Err(err) => {
            warn!("Failed to load map file, using default map file. {}", err);
            Tilemap::default()
        }
    };

    return map;
}
