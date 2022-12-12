use bevy::{
    app::AppExit,
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
    enums::*,
};


pub fn handle_main_menu_inputs(
    keys: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut state: ResMut<State<AppState>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut button_interaction: Query<
        (&Interaction, &ButtonInfo, &Children),
        (Changed<Interaction>, (With<Button>, With<MainMenu>)),
    >,
    mut text_query: Query<&mut Text>,
    mut exit: EventWriter<AppExit>,
) {
    if state.current() != &AppState::MainMenu { return; }

    for (interaction, info, children) in &mut button_interaction {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                info!("Pressed {:?} button", info.id);

                match info.id {
                    ButtonID::NewGame => {
                        match state.set(AppState::Playing) {
                            Ok(v) => info!("Switched into Playing state"),
                            Err(e) => warn!("Failed to switch into the playing state on button press"),
                        }
                    },
                    ButtonID::Exit => {
                        info!("Exiting game");
                        exit.send(AppExit);
                    },
                    _ => {},
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                info!("Hovering {:?} button", info.id);
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
            }
        }
    }

    for event in keyboard_events.iter() {
        match event.state {
            ButtonState::Pressed => {
                match event.key_code {
                    Some(key_code) => {
                        if key_code == KeyCode::Return {
                            match state.push(AppState::Playing) {
                                Ok(v) => info!("Switched into Playing state"),
                                Err(e) => warn!("Failed to switch into the playing state on Escape pressed"),
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
    //     info!("Switching to playing.");
    //     match state.set(AppState::Playing) {
    //         Ok(v) => {},
    //         Err(e) => warn!("Failed to switch into the plaing state on Escape pressed"),
    //     }
    // }


}
