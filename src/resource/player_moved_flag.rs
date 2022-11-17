use serde::{Deserialize, Serialize};
use log::{ info, warn };
use bevy::prelude::*;


#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct PlayerMovedFlag {
    pub moved_since_last_frame: bool
}

impl Default for PlayerMovedFlag {
    fn default() -> Self {
        Self { moved_since_last_frame: false }
    }
}
