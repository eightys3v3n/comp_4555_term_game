use bevy::{
    prelude::*,
    ui::UiImage,
};
use super::super::{
    resource::{
        config::{
            Config,
            ButtonConfig,
        },
    },
    component::*,
    enums::*,
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
                ..default()
            },
            MainMenu,
        ));
        // .with_children(|parent| {
        //     parent.spawn((
        //         NodeBundle {
        //             style: Style {
        //                 size: Size::new(Val::Percent(60.0), Val::Percent(100.0)),
        //                 ..default()
        //             },
        //             background_color: Color::rgb(1., 0., 0.).into(),
        //             ..default()
        //         },
        //     ));
        //     parent.spawn((
        //         NodeBundle {
        //             style: Style {
        //                 size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        //                 flex_direction: FlexDirection::Column,
        //                 ..default()
        //             },
        //             background_color: Color::rgb(0., 1., 0.).into(),
        //             ..default()
        //         },
        //     ))
        //     .with_children(|parent| {
        //         spawn_button(parent, &config, &config.menu.new_game, &asset_server);
        //         spawn_button(parent, &config, &config.menu.exit, &asset_server);
        //     });
        //     parent.spawn((
        //         NodeBundle {
        //             style: Style {
        //                 size: Size::new(Val::Percent(60.0), Val::Percent(100.0)),
        //                 ..default()
        //             },
        //             background_color: Color::rgb(1., 0., 0.).into(),
        //             ..default()
        //         },
        //     ));
        // });
}


pub fn hide(
    mut commands: Commands,
    mut ui_elements: Query<&mut Visibility, With<GameOver>>,
) {
    for (mut visibility) in ui_elements.iter_mut() {
        visibility.is_visible = false;
    }
}

pub fn show(
    mut commands: Commands,
    mut ui_elements: Query<&mut Visibility, With<GameOver>>,
) {
    for (mut visibility) in ui_elements.iter_mut() {
        visibility.is_visible = true;
    }
}
