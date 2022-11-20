use bevy::{
    prelude::*,
};
use log::warn;
use super::super::component::*;
use super::super::{
    resource::{ config::Config },
    enums::AppState,
};

pub fn handle_playing_inputs(
    keys: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut state: ResMut<State<AppState>>,
) {
    if state.current() != &AppState::Playing { return; }

    if keys.just_pressed(KeyCode::Escape) {
        info!("Switching to main menu.");
        match state.set(AppState::MainMenu) {
            Ok(v) => {},
            Err(e) => warn!("Failed to switch into the main menu state on Escape pressed"),
        }
    }
}
