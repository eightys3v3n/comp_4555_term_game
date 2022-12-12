use bevy::{
    prelude::*,
};
use super::super::{
    resource::{
        config::Config,
    },
    component::*,
    ui::*,
};


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                background_color: Color::rgb(0., 0., 1.).into(),
                ..default()
            },
            GameOver,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(60.0), Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgb(1., 0., 0.).into(),
                    ..default()
                },
            ));
            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                        flex_direction: FlexDirection::Column,
                        // justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0., 1., 0.).into(),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "This would be your score!",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.0, 0.0, 0.0),
                        font: asset_server.load(&config.menu.button_font),
                    },
                ));
                spawn_button(parent, &config, &config.menu.main_menu, &asset_server);
            });
            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(60.0), Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgb(1., 0., 0.).into(),
                    ..default()
                },
            ));
        });
}
