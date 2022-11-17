use bevy::{
    prelude::*,
};
mod system;
mod component;
mod resource;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<resource::config::Config>()
        .init_resource::<resource::tilemap::Tilemap>()
        .init_resource::<resource::player_moved_flag::PlayerMovedFlag>()
        .add_startup_system(setup)
        .add_startup_system(load_map)
        .add_system(system::map_scroll::map_scroll)
        // .add_system(system::example::print_keyboard_event_system)
        // .add_system(system::example::debug_key)
        // .add_system(system::example::print_mouse_events_system)
        // .add_system(system::example::mouse_click_system)
        // .add_system(system::example::grab_mouse)
        .add_system(system::player_movement::set_player_movements)
        .add_system(system::velocity::apply_velocity)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<resource::config::Config>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((component::Character {
        sprite: SpriteBundle {
            texture: asset_server.load(&config.player.image_path),
            transform: Transform::from_xyz(0.0, 0.0, config.player.default_z_height),
            sprite: Sprite {
                custom_size: Some(Vec2::new(config.player.width, config.player.height)),
                ..default()
            },
            ..default()
        },
        velocity: component::Velocity::new(0.0, 0.0),
        },
        component::Player,
    ));

}

fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<resource::config::Config>,
    tilemap: Res<resource::tilemap::Tilemap>,
) {
    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            let map_x: f32 = (x as f32 - tilemap.centre_x as f32) * config.map.tile_size as f32;
            let map_y: f32 = (y as f32 - tilemap.centre_y as f32) * config.map.tile_size as f32;

            commands.spawn((
                component::Tile,
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
