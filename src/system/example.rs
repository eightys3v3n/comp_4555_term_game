use bevy::{
    prelude::*,
    window::CursorGrabMode,
    input::keyboard::KeyboardInput,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
};
use super::super::resource::tilemap::Tilemap;


pub fn set_player_movements(
    keys: Res<Input<KeyCode>>,
    tilemap: Res<Tilemap>,
) {
    if keys.just_pressed(KeyCode::Q) {
        info!("Pressed Q");
        println!("Found {} tiles", tilemap.tiles.len());
    }
}

/// This system prints out all keyboard events as they come in
pub fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);
    }
}

/// This system prints out all mouse events as they come in
pub fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in mouse_button_input_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_motion_events.iter() {
        info!("{:?}", event);
    }

    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_wheel_events.iter() {
        info!("{:?}", event);
    }
}

// This system prints messages when you press or release the left mouse button:
pub fn mouse_click_system(mouse_button_input: Res<Input<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        info!("left mouse currently pressed");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}

pub fn grab_mouse(
    mut windows: ResMut<Windows>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.primary_mut();
    if mouse.just_pressed(MouseButton::Left) {
        window.set_cursor_visibility(false);
        window.set_cursor_grab_mode(CursorGrabMode::Locked);
    }
    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_visibility(true);
        window.set_cursor_grab_mode(CursorGrabMode::None);
    }
}
