mod system;
mod component;
mod resource;
mod enums;
mod state;
mod ui;

use log::{ info, warn };
use bevy::{
    prelude::*,
    time::FixedTimestep,
    window::*,
};
use resource::{
    config::Config,
    tilemap::Tilemap,
    player_moved_flag::PlayerMovedFlag,
};
use system::{
    enemy_movement::enemy_movement,
    player_movement::set_player_movements,
    velocity::apply_velocity,
    playing_inputs::handle_playing_inputs,
    main_menu_inputs::handle_main_menu_inputs,
    game_over_inputs::handle_game_over_inputs,
};
use enums::{
    AppState
};
use state::{ playing, main_menu, game_over };


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Untitled Game".to_string(),
                width: 800.,
                height: 500.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .init_resource::<Config>()
        .init_resource::<Tilemap>()
        .init_resource::<PlayerMovedFlag>()
        .add_state(AppState::MainMenu)
        .add_startup_system(setup)
        // .add_state(AppState::MainMenu)
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(main_menu::setup)
        )
        .add_system_set(
            SystemSet::on_resume(AppState::MainMenu)
                .with_system(main_menu::setup)
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(handle_main_menu_inputs)
        )
        .add_system_set(
            SystemSet::on_pause(AppState::MainMenu)
                .with_system(main_menu::teardown)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(main_menu::teardown)
        )
        .add_system_set(
            SystemSet::on_resume(AppState::GameOver)
                .with_system(game_over::setup)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::GameOver)
                .with_system(game_over::setup)
        )
        .add_system_set(
            SystemSet::on_update(AppState::GameOver)
                .with_system(handle_game_over_inputs)
        )
        .add_system_set(
            SystemSet::on_pause(AppState::GameOver)
                .with_system(game_over::teardown)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::GameOver)
                .with_system(game_over::teardown)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(playing::setup)
                .with_system(playing::load_map)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(enemy_movement)
                //.with_system(map_scroll)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(set_player_movements)
                .with_system(apply_velocity)
                .with_system(handle_playing_inputs)
        )
        .run();
}


pub fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    config: Res<Config>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut windows = windows.iter_mut().collect::<Vec<&mut Window>>();

    assert!(windows.len() > 0);
    if windows.len() > 1 {
        warn!("I am not expecting more than one window but there are {}", windows.len());
    }

    // windows[0].set_title(String::from("Untitled Game"));
    // windows[0].set_resolution(config.window.width, config.window.height);
}
