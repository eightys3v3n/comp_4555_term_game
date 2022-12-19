use bevy::prelude::*;


#[derive(Resource, Debug)]
pub struct Store {
    pub purchase_count_damage: i32,
    pub purchase_count_range: i32,
    pub purchase_count_fire_rate: i32,
    pub purchase_count_heal: i32,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            purchase_count_damage: 0,
            purchase_count_range: 0,
            purchase_count_fire_rate: 0,
            purchase_count_heal: 0,
        }
    }
}
