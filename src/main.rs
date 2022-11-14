use bevy::{
    prelude::*,
};
mod system;
mod component;
mod resource;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<resource::Config>()
        .add_startup_system(setup)
        .add_system(system::example::print_keyboard_event_system)
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
    config: Res<resource::Config>
) {
    // commands.insert_resource(resource::load_config());
    println!("Loaded image path {}", config.player.image_path);

    commands.spawn(Camera2dBundle::default());

    let player_id = commands.spawn((component::Character {
        sprite: SpriteBundle {
            texture: asset_server.load(&config.player.image_path),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        velocity: component::Velocity::new(0.0, 0.0),
        },
        component::Player,
    ));
}
