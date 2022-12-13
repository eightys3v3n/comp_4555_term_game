use bevy::{
    prelude::*,
};
use super::super::{
    component::*,
    resource::{
        config::Config,
        round::RoundInfo,
    },
    event::*,
};
use std::time::{
    SystemTime,
    Duration,
};


pub fn transition_rounds(
    config: Res<Config>,
    mut round: ResMut<RoundInfo>,
    mut round_start_events: EventWriter<RoundStartEvent>,
    mut round_end_events: EventWriter<RoundEndEvent>,
    enemies: Query<(), With<Enemy>>,
) {
    // End the round when there is no end time and the enemy count is 0.
    match round.end_time {
        None => {
            if enemies.is_empty() && round.enemy_counts.Basic == 0 {
                info!("No more enemies left, ending round {}", round.number);
                round.end_time = Some(SystemTime::now());
                round_end_events.send(RoundEndEvent{round_number: round.number});
            } else {
                info!("Still waiting for player to kill {} enemies.", round.enemy_counts.Basic + enemies.iter().len() as u64);
            }
        }
        Some(end_time) => {
            if end_time.elapsed().unwrap() >= Duration::from_secs(config.round.start_delay) {
                round.number += 1;
                round.end_time = None;
                round.start_time = Some(SystemTime::now());


                info!("Starting round {}", round.number);

                setup_round(&mut round);

                round_start_events.send(RoundStartEvent{round_number: round.number});
            } else {
                info!("Waiting for {} more seconds before starting round {}",
                      (Duration::from_secs(config.round.start_delay) - end_time.elapsed().unwrap()).as_secs(),
                      round.number + 1,
                );
            }
        }
    }
}


fn setup_round(round: &mut ResMut<RoundInfo>) {
    if round.number == 1 {
        round.enemy_counts.Basic = 5;
    } else {
        round.enemy_counts.Basic = 5 * u64::pow(2, round.number as u32);
    }
}
