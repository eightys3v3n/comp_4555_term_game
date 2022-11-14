use bevy::prelude::*;


#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity {
            x: 0.0,
            y: 0.0
        }
    }
}


#[derive(Bundle)]
pub struct Character {
    #[bundle]
    pub sprite: SpriteBundle,

    pub velocity: Velocity,
}
