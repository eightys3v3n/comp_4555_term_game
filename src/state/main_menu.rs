use bevy::prelude::*;
use super::super::{
    resource::{
        config::Config,
    },
    component::*,
    enums::*,
};


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    info!("Displaying buttons at {}x{}", window.width() / 2.0 - config.menu.r#continue.width, 0.0);

    commands.spawn((
        Button,
        SpriteBundle {
            texture: asset_server.load(&config.menu.r#continue.image_path),
            transform: Transform::from_xyz(window.width() / 2.0 - config.menu.r#continue.width / 2.0, 0.0, config.menu.r#continue.z_height),
            ..default()
        },
        // SpriteBundle {
        //     texture: asset_server.load(&config.menu.new_game.image_path),
        //     transform: Transform::from_xyz(window.width() / 2.0 - config.menu.new_game.width / 2.0, 0.0, config.menu.new_game.z_height),
        //     ..default()
        // },
        // SpriteBundle {
        //     texture: asset_server.load(&config.menu.exit.image_path),
        //     transform: Transform::from_xyz(window.width() / 2.0 - config.menu.exit.width / 2.0, 0.0, config.menu.exit.z_height),
        //     ..default()
        // },
    ));
}
