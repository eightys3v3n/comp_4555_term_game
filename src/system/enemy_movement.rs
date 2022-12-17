use bevy::{
    prelude::*,
};
use log::warn;
use super::super::{
    component::*,
    enums::*,
    resource::config::Config,
};
use std::collections::HashMap;

pub fn enemy_movement(
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies_query: Query<(Entity, &mut Transform, &Enemy, &mut Velocity, &CollideInfo), (With<Enemy>, Without<Player>)>,
    config: Res<Config>,
) {
    let mut enemies: Vec<(Entity, Mut<Transform>, &Enemy, Mut<Velocity>, &CollideInfo)> = enemies_query.iter_mut().collect();

    for (_, enemy_transform, enemy, enemy_velocity, _) in enemies.iter_mut() {
        for move_behaviour in enemy.move_behaviour.clone() {
            match move_behaviour {
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
                        Err(e) => {}
                    }
                },
                _ => {},
            }
        }
    }


    // Handle the AvoidEnemies movement type.
    // calculate how much each other enemy is pushing a given enemy.
    let mut enemy_pushes: HashMap::<Entity, Vec<Vec2>> = HashMap::new();
    for (entity_one, enemy_one_transform, enemy_one, enemy_one_velocity, enemy_one_collide_info) in enemies.iter() {
        if ! enemy_one.move_behaviour.contains(&MoveBehaviour::AvoidEnemies) { continue; }

        let force = Vec2::new(0., 0.,);

        if !enemy_pushes.contains_key(&entity_one) {
            enemy_pushes.insert(*entity_one, Vec::new());
        }
        let mut pushes = enemy_pushes.get_mut(entity_one).unwrap();

        for (entity_two, enemy_two_transform, enemy_two, enemy_two_velocity, enemy_two_collide_info) in enemies.iter() {
            if entity_one == entity_two { continue; }

            let distance = enemy_one_transform.translation.distance(enemy_two_transform.translation);
            let max_force_range = enemy_one_collide_info.radius + enemy_two_collide_info.radius;
            if distance > max_force_range { continue; }

            let power_of_force;
            if distance < 0.1 {
                power_of_force = 1.;
            } else {
                power_of_force = 1. - (distance / max_force_range);
            }

            let displ_vec3 = enemy_one_transform.translation - enemy_two_transform.translation;
            let displ_vec2 = Vec2::new(displ_vec3.x, displ_vec3.y).normalize_or_zero() * power_of_force;

            pushes.push(displ_vec2);
        }
    }

    // sum up the pushes on each enemy
    let mut summed_pushes: HashMap::<&Entity, Vec2> = HashMap::new();
    for (entity, pushes) in &enemy_pushes {
        if !summed_pushes.contains_key(entity) {
            summed_pushes.insert(entity, Vec2::new(0., 0.));
        }
        let summed_push = summed_pushes.get_mut(entity).unwrap();

        for push in pushes {
            *summed_push = Vec2::new(
                summed_push.x + push.x,
                summed_push.y + push.y,
            );
        }

        *summed_push = *summed_push * config.enemy.basic.move_speed;
    }

    // apply the pushes on each enemy so they walk just a bit away from their friends.
    for (entity, _, enemy, velocity, _) in enemies.iter_mut() {
        if ! enemy.move_behaviour.contains(&MoveBehaviour::AvoidEnemies) { continue; }

        match summed_pushes.get(entity) {
            Some(summed_push) => {
                let mut new_vel = *summed_push + Vec2::new(velocity.to_xy().0, velocity.to_xy().1);
                new_vel = new_vel.normalize_or_zero() * config.enemy.basic.move_speed;
                velocity.from_xy(new_vel.x, new_vel.y);
            },
            None => {
                warn!("Missing summed push for entity {:?}", entity);
            }
        };
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
