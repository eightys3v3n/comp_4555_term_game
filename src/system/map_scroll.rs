use bevy::{
    prelude::*,
};
use log::{ error };
use super::super::component::*;
use super::super::resource::player_moved_flag::PlayerMovedFlag;

pub fn map_scroll(
    camera           : Query<(&Transform, (With<Camera>, Without<Player>, Without<Tile>))>,
    player           : Query<(&Transform, (With<Player>, Without<Tile>, Without<Camera>))>,
    tiles            : Query<(&mut Transform, (With<Tile>, Without<Player>, Without<Camera>))>,
    mut player_moved : ResMut<PlayerMovedFlag>,
) {
    if ! player_moved.moved_since_last_frame {
        // println!("Player has not moved since last frame");
        return;
    }

    // info!("Player has moved since last frame");

    match camera.get_single() {
        Ok(_camera_transform) => {
            match player.get_single() {
                Ok(_player_transform) => {
                    player_moved.moved_since_last_frame = false;
                    for (_tile, _) in tiles.iter() {
                        // println!("Moving tile: ({}, {})", &tile.translation.x, &tile.translation.y)
                    }
                },
                Err(e) => error!("Error getting player Transform: {}", e)
            }
        },
        Err(e) => error!("Error getting camera Transform: {}", e)
    }
}
