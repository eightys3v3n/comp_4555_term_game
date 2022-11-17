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
        let vel_xy = velocity.to_xy();
        transform.translation.x += vel_xy.0 * time.delta_seconds();
        transform.translation.y += vel_xy.1 * time.delta_seconds();

        match player {
            Some(_) => {
                if velocity.to_xy().0 != 0.0 || velocity.to_xy().1 != 0.0 {
                    player_moved.moved_since_last_frame = true;
                }
            }
            None => {}
        };
    }
}
