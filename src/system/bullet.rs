use bevy::{
    prelude::*,
};
use log::warn;
use super::super::{
    component::*,
    enums::AppState,
    event::*,
};


pub fn fire_bullet(
    mut commands: Commands,
    mut fire_bullet_events: EventReader<FireBulletEvent>,
) {
    for event in fire_bullet_events.iter() {
        info!("Firing bullet {:?}", event.bullet_type);
    }
}
