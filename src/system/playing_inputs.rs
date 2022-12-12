use bevy::{
    prelude::*,
    input::{
        keyboard::KeyboardInput,
        ButtonState,
    },
};
use log::warn;
use super::super::{
    enums::AppState,
};

pub fn handle_playing_inputs(
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
                            match state.set(AppState::MainMenu) {
                                Ok(_) => info!("Switched into Main Menu state"),
                                Err(e) => warn!("Failed to switch into the Main Menu state on Escape pressed. {}", e),
                            }
                        } else if key_code == KeyCode::Grave { // ~ symbol
                            match state.set(AppState::GameOver) {
                                Ok(_) => info!("Switched into Game Over state"),
                                Err(e) => warn!("Failed to switch into the Game Over state on ` pressed. {}", e),
                            }
                        }
                    }
                    None => {}
                }
            },
            ButtonState::Released => {}
        }
    }
}
