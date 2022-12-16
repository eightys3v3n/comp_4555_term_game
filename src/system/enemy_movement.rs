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


/// Given a value between from_low and from_high, return the equivalent value if it were between to_low and to_high.
fn remap_range(val: f32, from_low: f32, from_high: f32, to_low: f32, to_high: f32) -> f32 {
    assert!(from_low < from_high);
    assert!(to_low < to_high);

    if (val * 10000.).round()/10000. == (from_low * 10000.).round()/10000. { return to_low; }
    if val < from_low { return to_low; }
    if val > from_high { return to_high; }

    let mut ret: f32;

    ret = val - from_low;
    ret = ret / (from_high - from_low);
    ret = ret * (to_high - to_low);
    ret += to_low;
    return ret;
}


#[cfg(test)]
mod tests {
    #[test]
    fn remap_range() {
        assert_eq!(10., super::remap_range(1., 1., 10., 10., 100.));
        assert_eq!(50., super::remap_range(5., 1., 10., 10., 100.));
        assert_eq!(0.5, super::remap_range(5., 0., 10., 0., 1.));
        assert_eq!(100., super::remap_range(10., 1., 10., 10., 100.));
        assert_eq!(50., super::remap_range(1., 0., 2., 0., 100.));
    }
}
