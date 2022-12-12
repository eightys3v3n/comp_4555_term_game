use bevy::{
    prelude::*,
    input::{
        keyboard::KeyboardInput,
        ButtonState,
    },
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
    mut keyboard_events: EventReader<KeyboardInput>,
) {
    if state.current() != &AppState::Playing { return; }

    for event in keyboard_events.iter() {
        match event.state {
            ButtonState::Pressed => {
                match event.key_code {
                    Some(key_code) => {
                        if key_code == KeyCode::Escape {
                            match state.pop() {
                                Ok(v) => info!("Switched into Main Menu state"),
                                Err(e) => warn!("Failed to switch into the main menu state on Escape pressed"),
                            }
                        }
                    }
                    None => {}
                }
            },
            ButtonState::Released => {}
        }
    }

    // if keys.just_pressed(KeyCode::Escape) {
    //     info!("Switching to main menu.");
    //     match state.set(AppState::MainMenu) {
    //         Ok(v) => {},
    //         Err(e) => warn!("Failed to switch into the main menu state on Escape pressed"),
    //     }
    // }
}
