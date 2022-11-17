use bevy::{
    prelude::*,
};
use super::super::component::*;
use super::super::resource::{
    config::Config,
};

pub fn set_player_movements(
    keys: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut query: Query<(&Player, &mut Velocity)>,
) {
    match query.get_single_mut() {
        Ok((_, mut velocity)) => {
            let (mut vel_x, mut vel_y) = velocity.to_xy();

            if keys.just_pressed(KeyCode::W) {
                info!("Pressed W");
                vel_y += config.player.move_speed;
            }
            else if keys.just_released(KeyCode::W) {
                info!("Released W");
                vel_y -= config.player.move_speed;
            }

            if keys.just_pressed(KeyCode::S) {
                info!("Pressed S");
                vel_y -= config.player.move_speed;
            }
            else if keys.just_released(KeyCode::S) {
                info!("Released S");
                vel_y += config.player.move_speed;
            }

            if keys.just_pressed(KeyCode::A) {
                info!("Pressed A");
                vel_x -= config.player.move_speed;
            }
            else if keys.just_released(KeyCode::A) {
                info!("Released A");
                vel_x += config.player.move_speed;
            }

            if keys.just_pressed(KeyCode::D) {
                info!("Pressed D");
                vel_x += config.player.move_speed;
            }
            else if keys.just_released(KeyCode::D) {
                info!("Released D");
                vel_x -= config.player.move_speed;
            }

            velocity.from_xy(vel_x, vel_y);
        },
        Err(err) => warn!("Error finding player for movement: {}", err)
    };
}
