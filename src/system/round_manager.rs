use bevy::{
    prelude::*,
};
use super::super::{
    component::*,
    resource::{
        config::Config,
        round::RoundInfo,
        counter::Counters,
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
    mut counters: ResMut<Counters>,
    enemies: Query<(), With<Enemy>>,
) {
    // End the round when there is no end time and the enemy count is 0.
    match round.end_time {
        None => {
            if enemies.is_empty() && round.enemy_counts.Basic == 0 {
                info!("No more enemies left, ending round {}", round.number);
                round.end_time = Some(SystemTime::now());
                round_end_events.send(RoundEndEvent{round_number: round.number});

                counters.points += round.number as f32 * 5.;
            }
        }
        Some(end_time) => {
            if end_time.elapsed().unwrap() >= Duration::from_secs(config.round.start_delay) {
                round.number += 1;
                round.end_time = None;
                round.start_time = Some(SystemTime::now());

                info!("Starting round {}", round.number);

                setup_round(&mut round, config);

                round_start_events.send(RoundStartEvent{round_number: round.number});
            }
        }
    }
}


fn setup_round(
    round: &mut ResMut<RoundInfo>,
    config: Res<Config>,
) {
    if round.number == 1 {
        round.enemy_counts.Basic = 5;
        round.enemy_counts.Tank = 0;
        return;
    }
    round.enemy_counts.Basic = (3. * round.number as f32 * config.round.basic_multiplier) as u64;

    if round.number >= 4 {
        round.enemy_counts.Tank = (1. * round.number as f32 * config.round.tank_multiplier) as u64;
        return;
    }

    round.enemy_counts.Tank = 0;
}
