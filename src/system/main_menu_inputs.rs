use bevy::{
    prelude::*,
    input::{
        keyboard::KeyboardInput,
        ButtonState,
        mouse::MouseButtonInput,
    },
};
use log::{ warn, info };
use super::super::component::*;
use super::super::{
    resource::{ config::Config },
    enums::AppState,
};


pub fn handle_main_menu_inputs(
    keys: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut state: ResMut<State<AppState>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut button_interaction: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    if state.current() != &AppState::MainMenu { return; }

    for (interaction, mut color, children) in &mut button_interaction {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
            }
        }
    }

    // for event in keyboard_events.iter() {
    //     match event.state {
    //         ButtonState::Pressed => {
    //             match event.key_code {
    //                 Some(key_code) => {
    //                     if key_code == KeyCode::Escape {
    //                         match state.set(AppState::Playing) {
    //                             Ok(v) => info!("Switched into Playing state"),
    //                             Err(e) => warn!("Failed to switch into the playing state on Escape pressed"),
    //                         }
    //                     }
    //                 }
    //                 None => {}
    //             }
    //         },
    //         ButtonState::Released => {}
    //     }
    // }

    // if keys.just_pressed(KeyCode::Escape) {
    //     info!("Switching to playing.");
    //     match state.set(AppState::Playing) {
    //         Ok(v) => {},
    //         Err(e) => warn!("Failed to switch into the plaing state on Escape pressed"),
    //     }
    // }


}
