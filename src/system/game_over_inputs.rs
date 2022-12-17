use bevy::{
    app::AppExit,
    prelude::*,
    input::{
        keyboard::KeyboardInput,
        ButtonState,
    },
};
use log::{ warn, info };
use super::super::component::*;
use super::super::{
    enums::*,
};


pub fn handle_game_over_inputs(
    mut state: ResMut<State<AppState>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut button_interaction: Query<
        (&Interaction, &ButtonInfo, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut exit: EventWriter<AppExit>,
) {
    // Need to pop self from state stack when leaving, then set to main menu.


    if state.current() != &AppState::GameOver {
        keyboard_events.clear();
        return;
    }

    for (interaction, info, children) in &mut button_interaction {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                // text.sections[0].value = "Press".to_string();
                info!("Pressed {:?} button", info.id);

                match info.id {
                    ButtonID::MainMenu => {
                        match state.set(AppState::MainMenu) {
                            Ok(_) => info!("Sweitched to Main Menu."),
                            Err(e) => warn!("Failed to switch to Main Menu on press. {}", e),
                        };
                    },
                    ButtonID::Exit => {
                        info!("Exiting game");
                        exit.send(AppExit);
                    },
                    _ => {},
                }
            }
            _ => {},
            // Interaction::Hovered => {
            //     text.sections[0].value = "Hover".to_string();
            //     info!("Hovering {:?} button", info.id);
            // }
            // Interaction::None => {
            //     text.sections[0].value = "Button".to_string();
            // }
        }
    }

    for event in keyboard_events.iter() {
        match event.state {
            ButtonState::Pressed => {
                match event.key_code {
                    Some(key_code) => {
                        if key_code == KeyCode::Return {
                            match state.set(AppState::MainMenu) {
                                Ok(_) => info!("Sweitched to Main Menu."),
                                Err(e) => warn!("Failed to switch to Main Menu on press. {}", e),
                            };
                        }
                    }
                    None => {}
                }
            },
            ButtonState::Released => {}
        }
    }
}
