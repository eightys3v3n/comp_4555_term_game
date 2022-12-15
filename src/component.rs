use bevy::prelude::*;
use super::enums::*;
use std::fmt;


pub mod velocity;
pub use velocity::Velocity;

#[derive(Component, Debug)]
pub struct ButtonInfo {
    pub id: ButtonID,
}

#[derive(Component, Debug)]
pub struct BulletInfo {
    pub r#type: BulletType,
    pub range: f32,
    pub size: f32,
    pub damage: f32,
    pub start_transform: Transform,
}

#[derive(Component, Debug)]
pub struct Enemy {
    pub move_behaviour: MoveBehaviour,
}

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "stuff")
    }
}


#[derive(Component, Debug)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.current, self.max)
    }
}

impl Health {
    pub fn to_string(&self) -> String {
        return format!("{}/{}", self.current, self.max);
    }
}

impl Health {
    pub fn new(max: f32) -> Health {
        return Health {
            max: max,
            current: max,
        };
    }
}


// Tagging
#[derive(Component, Debug)]
pub struct Player;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The Player")
    }
}

#[derive(Component, Debug)]
pub struct MainMenu;

#[derive(Component, Debug)]
pub struct GameOver;

#[derive(Component, Debug)]
pub struct Tile;


// Bundles
#[derive(Bundle)]
pub struct Character {
    #[bundle]
    pub sprite: SpriteBundle,

    pub velocity: Velocity,
    pub health: Health,
}

#[derive(Bundle)]
pub struct BulletBundle {
    #[bundle]
    pub sprite: SpriteBundle,

    pub velocity: Velocity,
    pub bullet_info: BulletInfo,
}
