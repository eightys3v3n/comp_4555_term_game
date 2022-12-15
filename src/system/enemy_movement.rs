use bevy::{
    prelude::*,
};
use log::warn;
use super::super::{
    component::*,
    enums::*,
    resource::config::Config,
};

pub fn enemy_movement(
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<(&mut Transform, &Enemy, &mut Velocity), (With<Enemy>, Without<Player>)>,
    config: Res<Config>,
) {
    for (mut enemy_transform, enemy, mut enemy_velocity) in enemies.iter_mut() {
        match enemy.move_behaviour {
            MoveBehaviour::PointedToPlayer => {
                match player.get_single() {
                    Ok((player_transform)) => {
                        let displ_y = player_transform.translation.y - enemy_transform.translation.y;
                        let displ_x = player_transform.translation.x - enemy_transform.translation.x;
                        let displ_vel = Velocity::new_xy(displ_x, displ_y);

                        enemy_transform.rotation = Quat::from_rotation_z(displ_vel.direction.to_radians());
                        enemy_velocity.direction = displ_vel.direction;

                        match enemy.r#type {
                            EnemyType::Basic => enemy_velocity.scalar = config.enemy.basic.move_speed,
                            _ => warn!("Unimplemented enemy movement speed; moving some default amount"),
                        }
                    }
                    Err(e) => warn!("Couldn't use the player's position to direct enemy movements because there is no player? {}", e)
                }

            }
        }
    }
}
