use bevy::prelude::*;
use super::enums::MoveBehaviour;


pub mod velocity;
pub use velocity::Velocity;


// Tagging
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ButtonInfo {
    pub id: ButtonID,
}

#[derive(Component)]
pub struct Tile;


#[derive(Component)]
pub struct Enemy {
    pub move_behaviour: MoveBehaviour,
}


// Bundles
#[derive(Bundle)]
pub struct Character {
    #[bundle]
    pub sprite: SpriteBundle,

    pub velocity: Velocity,
}
