use bevy::prelude::*;


#[derive(Resource, Debug)]
pub struct Counters {
    pub enemies_killed: u64,
    pub points: f32,
}

impl Default for Counters {
    fn default() -> Self {
        Self { enemies_killed: 0, points: 0., }
    }
}
