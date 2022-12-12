use serde::{Deserialize, Serialize};
use std::{
    fs,
};
use log::{info, warn};
use bevy::prelude::*;


#[derive(Debug, Deserialize, Serialize)]
pub enum TileType {
    Grass,
}

#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct Tilemap {
    pub tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
    pub centre_x: u64,
    pub centre_y: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tile {
    pub r#type: TileType,
}


pub enum TilemapError {
    XOutsideMap,
    NoSuchTile,
}


impl Default for Tilemap {
    fn default() -> Self {
        load_map(String::from("world_1"))
    }
}

impl Tilemap {
    pub fn new(width: usize, height: usize, centre_x: u64, centre_y: u64) -> Self {
        let mut tiles: Vec<Tile> = Vec::with_capacity((width * height) as usize);

        for _ in 0..height {
            for _ in 0..width {
                tiles.push(Tile::new(TileType::Grass));
            }
        }

        Self {
            tiles,
            width,
            height,
            centre_x,
            centre_y,
        }
    }

    pub fn map_pos_to_array_pos(&self, _x: i64, _y: i64) -> (u64, u64) {
        return (10, 10);
    }

    pub fn array_pos_to_map_pos(&self, _x: u64, _y: u64) -> (i64, i64) {
        return (10, 10);
    }

    pub fn get_tile(&self, x: i64, y: i64) -> Result<&Tile, TilemapError> {
        if x >= self.width as i64 {
            // return Err(format!("Specified X is greater than tilemap width: {}>{}", x, self.width));
            return Err(TilemapError::XOutsideMap);
        }

        let (new_x, new_y) = self.map_pos_to_array_pos(x, y);
        let index: usize = (new_y * self.width as u64 * new_x) as usize;

        return match self.tiles.get(index) {
            None => {
                Err(
                    // format!("No tile for given position ({}, {}) -> ({}, {}) -> index({})", x, y, new_x, new_y, index)
                    TilemapError::NoSuchTile
                )
            },
            Some(t) => { Ok(t) }
        };
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
    let map_path: String = format!("assets/{}.ron", name);

    info!("Loading map file {}", map_path);

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
            Tilemap::new(11, 11, 5, 5)
        }
    };

    if map.height * map.width != map.tiles.len() {
        panic!("Map {} does not contain the correct number of tiles; {} when expected {}", name, map.tiles.len(), map.height * map.width);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
}
