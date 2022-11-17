use bevy::{
    prelude::*,
};
use super::super::component::*;
use super::super::resource::player_moved_flag::PlayerMovedFlag;


pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, Option<&Player>)>,
    mut player_moved: ResMut<PlayerMovedFlag>,
) {
    for (mut transform, velocity, player) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();

        match player {
            Some(_) => {
                if velocity.x != 0.0 || velocity.y != 0.0 {
                    player_moved.moved_since_last_frame = true;
                }
            }
            None => {}
        };
    }
}
