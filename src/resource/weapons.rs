use bevy::prelude::*;
use super::super::{
    enums::*,
};


#[derive(Resource, Debug)]
pub struct Weapons {
    pub ammo: Option<u64>,
    pub bullet_type: BulletType,
    pub weapon: WeaponType,
}

impl Default for Weapons {
    fn default() -> Self {
        Self {
            ammo: None,
            bullet_type: BulletType::Basic,
            weapon: WeaponType:: Pistol,
        }
    }
}
