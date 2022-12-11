use bevy::{
    prelude::*,
    ui::UiImage,
};
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

    // commands.spawn((
    //     Button,
    //     SpriteBundle {
    //         texture: asset_server.load(&config.menu.new_game.image_path),
    //         transform: Transform::from_xyz(0.0, 0.0, config.menu.new_game.z_height),
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(config.menu.new_game.width, config.menu.new_game.height)),
    //             ..default()
    //         },
    //         ..default()
    //     },
    // ));
    // commands.spawn((
    //     Button,
    //     SpriteBundle {
    //         texture: asset_server.load(&config.menu.exit.image_path),
    //         transform: Transform::from_xyz(0.0, -100.0, config.menu.exit.z_height),
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(config.menu.exit.width, config.menu.exit.height)),
    //             ..default()
    //         },
    //         ..default()
    //     },
    // ));

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(config.menu.new_game.width), Val::Px(config.menu.new_game.height)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            image: UiImage(asset_server.load(&config.menu.new_game.image_path)),
            background_color: Color::rgb(5.0, 5.0, 5.0).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                &config.menu.new_game.text,
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.0, 0.0, 0.0),
                    font: asset_server.load(&config.menu.button_font),
                },
            ));
        });
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(config.menu.exit.width), Val::Px(config.menu.exit.height)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(5.0, 5.0, 5.0).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                &config.menu.new_game.text,
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.0, 0.0, 0.0),
                    font: asset_server.load(&config.menu.button_font),
                },
            ));
        });
}
