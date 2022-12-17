use bevy::{
    prelude::*,
};
use log::warn;
use super::super::{
    component::*,
    enums::AppState,
    resource::counter::Counters,
};


pub fn character_killer(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    characters: Query<(Entity, &Health, Option<&Enemy>, Option<&Player>)>,
    mut counters: ResMut<Counters>,
) {
    for (character, health, enemy, player) in characters.iter() {
        if health.current <= 0. {
            match enemy {
                Some(enemy_info) => {
                    commands.entity(character).despawn_recursive();
                    counters.enemies_killed += 1;
                    counters.points += enemy_info.points;
                    info!("Points now {}", counters.points);
                    continue;
                },
                None => {}
            };

            match player {
                Some(_) => {
                    commands.entity(character).despawn_recursive();
                    info!("The player died.");
                    match state.set(AppState::GameOver) {
                        Ok(_) => info!("Switched into Game Over state"),
                        Err(e) => warn!("Failed to switch into the Game Over state on ` pressed. {}", e),
                    }
                    continue;
                }
                None => {}
            };
        }
    }
}
