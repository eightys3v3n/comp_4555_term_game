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
