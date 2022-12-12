use bevy::{
    prelude::*,
};
use super::{
    resource::{
        config::{
            Config,
            ButtonConfig,
        },
    },
    component::*,
};

pub fn spawn_button(parent: &mut ChildBuilder<'_, '_, '_>,
                config: &Res<Config>,
                button_config: &ButtonConfig,
                asset_server: &Res<AssetServer>) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(100.),
                                Val::Px(100.)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            // image: UiImage(asset_server.load(&config.menu.new_game.image_path)),
            background_color: Color::rgb(5.0, 5.0, 5.0).into(),
            ..default()
        },
        ButtonInfo {
            id: button_config.id.clone(),
        },
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            &button_config.text,
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(0.0, 0.0, 0.0),
                font: asset_server.load(&config.menu.button_font),
            },
        ));
    });
}
