use bevy::{
    prelude::*,
};
use log::warn;
use super::super::{
    component::*,
    enums::MoveBehaviour,
};

pub fn enemy_movement(
    player: Query<(&Transform, (With<Player>, Without<Enemy>))>,
    mut enemies: Query<(&mut Transform, &Enemy, &mut Velocity, (With<Enemy>, Without<Player>))>,
) {
    for (mut enemy_transform, enemy, mut enemy_velocity, _) in enemies.iter_mut() {
        match enemy.move_behaviour {
            MoveBehaviour::PointedToPlayer => {
                match player.get_single() {
                    Ok((player_transform, _)) => {
                        let displ_y = player_transform.translation.y - enemy_transform.translation.y;
                        let displ_x = player_transform.translation.x - enemy_transform.translation.x;
                        let displ_vel = Velocity::new_xy(displ_x, displ_y);

                        enemy_transform.rotation = Quat::from_rotation_z(displ_vel.direction.to_radians());
                        enemy_velocity.direction = displ_vel.direction;
                    }
                    Err(e) => warn!("Couldn't use the player's position to direct enemy movements: {}", e)
                }

            }
        }
    }
}
