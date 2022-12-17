use bevy::{
    prelude::*,
};
use super::super::{
    resource::{
        config::Config,
        round::RoundInfo,
        counter::Counters,
    },
    component::*,
    ui::*,
};


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    round: Res<RoundInfo>,
    counters: Res<Counters>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                // background_color: Color::rgb(0., 0., 1.).into(),
                ..default()
            },
            GameOver,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                        ..default()
                    },
                    // background_color: Color::rgb(1., 0., 0.).into(),
                    ..default()
                },
            ));
            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(40.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        // align_content: AlignContent::Center,
                        ..default()
                    },
                    // background_color: Color::rgb(0., 1., 0.).into(),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Px(100.)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::rgba(0., 0., 0., 0.7).into(),
                        ..default()
                    },
                )
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("Made it to round {}", round.number),
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(1., 1., 1.),
                            font: asset_server.load(&config.menu.button_font),
                        }
                    ).with_text_alignment(TextAlignment::CENTER)
                    );
                    parent.spawn(TextBundle::from_section(
                        format!("Killed {} enemies", counters.enemies_killed),
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(1., 1., 1.),
                            font: asset_server.load(&config.menu.button_font),
                        },
                    ));
                });
                parent.spawn(
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Px(100.)),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        // background_color: Color::rgb(1., 1., 0.).into(),
                        ..default()
                    },
                )
                .with_children(|parent| {
                    spawn_button(parent, &config, &config.menu.main_menu, &asset_server);
                });
            });
            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                        ..default()
                    },
                    // background_color: Color::rgb(1., 0., 0.).into(),
                    ..default()
                },
            ));
        });
}
