use bevy::prelude::*;
use super::super::{
    resource::{
        config::Config,
        tilemap::Tilemap
    },
    enums::*,
    component::*,
};
use std::collections::HashMap;


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    entities: Query<(Entity, AnyOf<(With<Player>, With<Enemy>)>)>,
    audio: Res<Audio>,
) {
    // Despawn all existing game stuff that needs to be reset.
    if ! entities.is_empty() {
        for (entity, _) in entities.iter() {
            commands.entity(entity).despawn();
        }
    }

    // Spawn in all game stuff.
    commands.spawn((
        Character {
            sprite: SpriteBundle {
                texture: asset_server.load(&config.player.image_path),
                transform: Transform::from_xyz(0.0, 0.0, config.player.z_height),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(config.player.width, config.player.height)),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity::new(0.0, 0.0),
            health: Health::new(config.player.default_health),
            collide_info: CollideInfo {
                radius: (config.player.width.max(config.player.height) / 2.) * 0.9,
                entity_type: EntityType::Player,
            },
        },
        Player,
    ));

    commands.spawn((
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        },
        Playing,
    ))
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(80.0), Val::Percent(100.0)),
                // border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            // background_color: Color::rgb(0., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    &config.window.round_counter_text,
                    TextStyle {
                        font_size: 30.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                        font: asset_server.load(&config.menu.button_font),
                    },
                ),
                HUD,
                UpdatableTextField {
                    field: TextField::RoundCounter,
                },
            ));
            parent.spawn((
                TextBundle::from_section(
                    &config.window.enemies_counter_text,
                    TextStyle {
                        font_size: 30.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                        font: asset_server.load(&config.menu.button_font),
                    },
                ),
                HUD,
                UpdatableTextField {
                    field: TextField::EnemiesCounter,
                },
            ));
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        &config.window.health_text,
                        TextStyle {
                            font_size: 30.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                            font: asset_server.load(&config.menu.button_font),
                        },
                    ),
                    TextSection::new(
                        format!("{}/{}", 1, 1),
                        TextStyle {
                            font_size: 30.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                            font: asset_server.load(&config.menu.button_font),
                        },
                    ),
                ]),
                HUD,
                UpdatableTextField {
                    field: TextField::Health,
                },
            ));
            parent.spawn((
                TextBundle::from_section(
                    &config.window.points_counter_text,
                    TextStyle {
                        font_size: 30.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                        font: asset_server.load(&config.menu.button_font),
                    },
                ),
                HUD,
                UpdatableTextField {
                    field: TextField::PointsCounter,
                },
            ));
        });

        parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Percent(100.0)),
                // border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            // background_color: Color::rgb(0., 0., 1.).into(),
            ..default()
        });

        parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(170.0), Val::Percent(100.0)),
                // border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            // background_color: Color::rgb(0., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            let mut fields: HashMap<TextField, Vec<TextSection>> = HashMap::new();
            for (modifier, modifier_config) in &config.store.modifiers {
                fields.insert(
                    modifier_config.field,
                    vec!(
                        TextSection::new(
                            format!("{}", &modifier_config.text),
                            TextStyle {
                                font: asset_server.load(&config.menu.button_font),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::new(
                            format!("${}\n", &modifier_config.cost),
                            TextStyle {
                                font: asset_server.load(&config.menu.button_font),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                    ),
                );
            }
            for (text_field, text_sections) in &fields {
                parent.spawn((
                    TextBundle::from_sections(text_sections.into_iter().cloned()),
                    UpdatableTextField {
                        field: *text_field,
                    },
                    HUD,
                ));
            }
        });
    });

    audio.play_with_settings(
        asset_server.load(&config.sound.background_music_path),
        PlaybackSettings::LOOP.with_volume(0.5),
    );
}

pub fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    tilemap: Res<Tilemap>,
    tiles: Query<Entity, With<Tile>>,
) {
    if ! tiles.is_empty() {
        for entity in tiles.iter() {
            commands.entity(entity).despawn();
        }
    }

    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            let map_x: f32 = (x as f32 - tilemap.centre_x as f32) * config.map.tile_size as f32;
            let map_y: f32 = (y as f32 - tilemap.centre_y as f32) * config.map.tile_size as f32;

            commands.spawn((
                Tile,
                SpriteBundle {
                    texture: asset_server.load(&config.map.grass_texture_path),
                    transform: Transform::from_xyz(map_x,
                                                   map_y,
                                                   config.map.default_z_height),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(config.map.tile_size, config.map.tile_size)),
                        ..default()
                    },
                    ..default()
                }
            ));
        }
    }
}
