use bevy::{
    prelude::*,
};
use log::warn;
use super::super::{
    component::*,
    enums::AppState,
};


pub fn character_killer(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    characters: Query<(Entity, &Health, Option<&Player>)>,
) {
    for (character, health, player) in characters.iter() {
        if health.current <= 0. {
            match player {
                Some(_) => {
                    commands.entity(character).despawn_recursive();
                    info!("The player died.");
                    match state.set(AppState::GameOver) {
                        Ok(_) => info!("Switched into Game Over state"),
                        Err(e) => warn!("Failed to switch into the Game Over state on ` pressed. {}", e),
                    }
                }
                None => commands.entity(character).despawn_recursive()
            };
        }
    }
}
