use bevy::{
    prelude::*,
};
use log::warn;
use super::super::{
    component::*,
    enums::*,
    event::*,
    resource::config::{ Config, BulletConfig },
};
use std::time::{ Duration, SystemTime };


pub fn detect_collisions(
    mut collision_events: EventWriter<CollideEvent>,
    collidable_query: Query<(Entity, &Transform, &CollideInfo), With<Transform>>,
    config: Res<Config>,
) {
    let collidables: Vec<(Entity, &Transform, &CollideInfo)> = collidable_query.iter().collect();
    for (first_entity, first_transform, first_collide_info) in collidables.iter() {
        for (second_entity, second_transform, second_collide_info) in collidables.iter() {
            if first_entity == second_entity { continue; }

            if first_transform.translation.distance(second_transform.translation) <= first_collide_info.radius + second_collide_info.radius {
                collision_events.send(CollideEvent {
                    from_entity_id: *first_entity,
                    from_entity_type: first_collide_info.entity_type,
                    to_entity_id: *second_entity,
                    to_entity_type: second_collide_info.entity_type,
                });
            }
        }
    }
}

pub fn do_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollideEvent>,
    mut bullet_info_query: Query<&mut BulletInfo>,
    mut health_query: Query<&mut Health>,
    mut enemy_query: Query<&mut Enemy>,
    config: Res<Config>,
) {
    // Have to implement both the case where a bullet hits an enemy and an enemy hits a bullet.

    for event in collision_events.iter() {
        if event.from_entity_type == EntityType::Bullet && event.to_entity_type == EntityType::Bullet {
            // ignore bullet-bullet collisions
            return;
        } else if event.from_entity_type == EntityType::Player && event.to_entity_type == EntityType::Bullet {
            // ignore player-bullet collisions because bullets fire from inside the player.
            return;
        } else if event.from_entity_type == EntityType::Bullet && event.to_entity_type == EntityType::Player {
            // ignore player-bullet collisions because bullets fire from inside the player.
            return;


        } else if event.from_entity_type == EntityType::Bullet && event.to_entity_type == EntityType::Enemy {
            let mut enemy_health = match health_query.get_mut(event.to_entity_id) {
                Ok(enemy_health) => enemy_health,
                Err(e) => {
                    warn!("Found no player health for this player entity? {}", e);
                    continue
                }
            };
            let mut bullet_info = match bullet_info_query.get_mut(event.from_entity_id) {
                Ok(bullet_info) => bullet_info,
                Err(e) => {
                    warn!("Found no bullet info for this bullet entity? {}", e);
                    continue
                }
            };

            enemy_health.current -= bullet_info.damage;

            info!("Hit an enemy for {} damage! {} health left", bullet_info.damage, enemy_health.current);
            bullet_info.hit_something = true;
            commands.entity(event.from_entity_id).despawn_recursive();
        } else if event.from_entity_type == EntityType::Enemy && event.to_entity_type == EntityType::Bullet {
            let mut enemy_health = match health_query.get_mut(event.from_entity_id) {
                Ok(enemy_health) => enemy_health,
                Err(e) => {
                    warn!("Found no player health for this player entity? {}", e);
                    continue
                }
            };
            let mut bullet_info = match bullet_info_query.get_mut(event.to_entity_id) {
                Ok(bullet_info) => bullet_info,
                Err(e) => {
                    warn!("Found no bullet info for this bullet entity? {}", e);
                    continue
                }
            };

            enemy_health.current -= bullet_info.damage;

            info!("Hit an enemy for {} damage! {} health left", bullet_info.damage, enemy_health.current);
            bullet_info.hit_something = true;
            commands.entity(event.to_entity_id).despawn_recursive();


        } else if event.from_entity_type == EntityType::Player && event.to_entity_type == EntityType::Enemy {
            let mut player_health = match health_query.get_mut(event.from_entity_id) {
                Ok(player_health) => player_health,
                Err(e) => {
                    warn!("Found no player health for this player entity? {}", e);
                    continue
                }
            };
            let mut enemy_info = match enemy_query.get_mut(event.to_entity_id) {
                Ok(enemy_info) => enemy_info,
                Err(e) => {
                    warn!("Found no enemy_info for this enemy entity? {}", e);
                    continue
                }
            };

            match enemy_info.last_hit_time {
                Some(last_hit_time) => {
                    if last_hit_time.elapsed().unwrap() < Duration::from_millis(config.enemy.basic.hit_cooldown) {
                        // Enemy can't hit stuff yet, ignore and process next collision.
                        continue
                    }
                }
                None => {
                    enemy_info.last_hit_time = Some(SystemTime::now())
                }
            };

            let damage = match enemy_info.r#type {
                EnemyType::Basic => config.enemy.basic.damage,
                _ => {
                    warn!("Unimplemented enemy type so couldn't determine damage to do to the player");
                    continue
                }
            };

            player_health.current -= damage;

            info!("Player ran into a {:?} Enemy which does {} damage.", enemy_info.r#type, damage);
        } else if event.from_entity_type == EntityType::Enemy && event.to_entity_type == EntityType::Player {
            let mut player_health = match health_query.get_mut(event.to_entity_id) {
                Ok(player_health) => player_health,
                Err(e) => {
                    warn!("Found no player health for this player entity? {}", e);
                    continue
                }
            };
            let mut enemy_info = match enemy_query.get_mut(event.from_entity_id) {
                Ok(enemy_info) => enemy_info,
                Err(e) => {
                    warn!("Found no enemy_info for this enemy entity? {}", e);
                    continue
                }
            };

            match enemy_info.last_hit_time {
                Some(last_hit_time) => {
                    if last_hit_time.elapsed().unwrap() < Duration::from_millis(config.enemy.basic.hit_cooldown) {
                        // Enemy can't hit stuff yet, ignore and process next collision.
                        continue
                    }
                }
                None => {
                    enemy_info.last_hit_time = Some(SystemTime::now())
                }
            };

            let damage = match enemy_info.r#type {
                EnemyType::Basic => config.enemy.basic.damage,
                _ => {
                    warn!("Unimplemented enemy type so couldn't determine damage to do to the player");
                    continue
                }
            };

            player_health.current -= damage;

            info!("Player ran into a {:?} Enemy which does {} damage.", enemy_info.r#type, damage);


        } else if event.from_entity_type == EntityType::Enemy && event.to_entity_type == EntityType::Enemy {
            // info!("Enemy ran into an enemy");


        } else {
            warn!("Unimplemented collision between {:?} and {:?}", event.from_entity_type, event.to_entity_type);
        }
    }
}
