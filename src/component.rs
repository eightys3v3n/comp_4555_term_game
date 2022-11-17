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
            x: x,
            y: y
        }
    }
}

#[derive(Component)]
pub struct Tile;

#[derive(Bundle)]
pub struct Character {
    #[bundle]
    pub sprite: SpriteBundle,

    pub velocity: Velocity,
}
