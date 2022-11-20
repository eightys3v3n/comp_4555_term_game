use bevy::{
    prelude::*,
};
use log::warn;
use super::super::component::*;
use super::super::{
    resource::{ config::Config },
    enums::AppState,
};

pub fn handle_main_menu_inputs(
    keys: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut state: ResMut<State<AppState>>,
) {
    if state.current() != &AppState::MainMenu { return; }

    if keys.just_pressed(KeyCode::Escape) {
        info!("Switching to playing.");
        match state.set(AppState::Playing) {
            Ok(v) => {},
            Err(e) => warn!("Failed to switch into the plaing state on Escape pressed"),
        }
    }
}
