use bevy::{
    prelude::*,
};
mod config;
mod system;

const CONFIG_FILE: &str = "config.ron";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(system::example::print_keyboard_event_system)
        .add_system(system::example::print_mouse_events_system)
        .add_system(system::example::mouse_click_system)
        .add_system(system::example::grab_mouse)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let conf = config::load_config(CONFIG_FILE);
    println!("{}", conf.player.image_path);

    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load(conf.player.image_path),
        ..default()
    });
}
