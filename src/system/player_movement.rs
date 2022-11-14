use bevy::{
    prelude::*,
};
use super::super::component::*;
use super::super::resource::config::Config;

pub fn set_player_movements(
    keys: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut query: Query<(&Player, &mut Velocity)>,
) {
    match query.get_single_mut() {
        Ok((_, mut velocity)) => {
            if keys.just_pressed(KeyCode::W) {
                info!("Pressed W");
                velocity.y += config.player.move_speed;
            }
            else if keys.just_released(KeyCode::W) {
                info!("Released W");
                velocity.y -= config.player.move_speed;
            }

            if keys.just_pressed(KeyCode::S) {
                info!("Pressed S");
                velocity.y -= config.player.move_speed;
            }
            else if keys.just_released(KeyCode::S) {
                info!("Released S");
                velocity.y += config.player.move_speed;
            }

            if keys.just_pressed(KeyCode::A) {
                info!("Pressed A");
                velocity.x -= config.player.move_speed;
            }
            else if keys.just_released(KeyCode::A) {
                info!("Released A");
                velocity.x += config.player.move_speed;
            }

            if keys.just_pressed(KeyCode::D) {
                info!("Pressed D");
                velocity.x += config.player.move_speed;
            }
            else if keys.just_released(KeyCode::D) {
                info!("Released D");
                velocity.x -= config.player.move_speed;
            }
        },
        Err(err) => warn!("Error finding player for movement: {}", err)
    };
}
