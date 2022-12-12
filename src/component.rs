use bevy::prelude::*;
use super::enums::*;


pub mod velocity;
pub use velocity::Velocity;

#[derive(Component)]
pub struct ButtonInfo {
    pub id: ButtonID,
}

#[derive(Component)]
pub struct Enemy {
    pub move_behaviour: MoveBehaviour,
}



// Tagging
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct Tile;


// Bundles
#[derive(Bundle)]
pub struct Character {
    #[bundle]
    pub sprite: SpriteBundle,

    pub velocity: Velocity,
}
