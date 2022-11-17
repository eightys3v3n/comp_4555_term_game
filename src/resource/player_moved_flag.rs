use bevy::prelude::*;


#[derive(Resource, Debug)]
pub struct PlayerMovedFlag {
    pub moved_since_last_frame: bool
}

impl Default for PlayerMovedFlag {
    fn default() -> Self {
        Self { moved_since_last_frame: false }
    }
}
