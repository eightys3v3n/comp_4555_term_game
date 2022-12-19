use bevy::prelude::*;
use super::super::{
    enums::*,
};


#[derive(Resource, Debug)]
pub struct Weapons {
    pub ammo: Option<u64>,
    pub bullet_type: BulletType,
    pub weapon: WeaponType,
    pub damage_modifier: f32,
    pub range_modifier: f32,
}

impl Default for Weapons {
    fn default() -> Self {
        Self {
            ammo: None,
            bullet_type: BulletType::Basic,
            weapon: WeaponType:: Pistol,
            damage_modifier: 1.0,
            range_modifier: 1.0,
        }
    }
}
