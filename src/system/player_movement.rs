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
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    if ! query.is_empty() {
        match query.get_single_mut() {
            Ok((mut velocity, mut transform)) => {
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

                // Round to certain number of decimal places
                vel_x = (vel_x * 10000.0).round() / 10000.0;
                vel_y = (vel_y * 10000.0).round() / 10000.0;

                let mut rotation_angle;
                if vel_x != 0. || vel_y != 0. {
                    rotation_angle = vel_y.atan2(vel_x);
                } else {
                    rotation_angle = 0.;
                }

                transform.rotation = Quat::from_rotation_z(rotation_angle);
                velocity.from_xy(vel_x, vel_y);
            },
            Err(err) => warn!("Error finding player for movement: {}", err)
        };
    }
}

