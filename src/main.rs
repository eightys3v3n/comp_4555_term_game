mod system;
mod component;
mod resource;
mod enums;
mod state;

use bevy::{
    prelude::*,
    time::FixedTimestep,
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
};
use enums::{
    AppState
};
use state::{ playing, main_menu };


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Config>()
        .init_resource::<Tilemap>()
        .init_resource::<PlayerMovedFlag>()
        .add_state(AppState::Playing)
        .add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(playing::setup)
                .with_system(playing::load_map)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(playing::setup)
                // .with_system(button_press)
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
        )
        .run();
}
