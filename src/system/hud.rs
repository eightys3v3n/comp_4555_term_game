use bevy::{
    prelude::*,
};
use log::warn;
use super::super::{
    component::*,
    enums::*,
    event::*,
    resource::{
        config::Config,
        round::RoundInfo,
        counter::Counters,
    },
};


pub fn updater(
    mut query: Query<(&mut Text, Option<&RoundCounter>, Option<&EnemiesCounter>, Option<&PointsCounter>), With<HUD>>,
    round_info: Res<RoundInfo>,
    config: Res<Config>,
    asset_server: Res<AssetServer>,
    counters: Res<Counters>,
) {
    for (mut text, maybe_round_counter, maybe_enemies_counter, maybe_points_counter) in query.iter_mut() {
        if ! maybe_round_counter.is_none() {
            text.sections[0].value = format!("{}{}", config.window.round_counter_text, round_info.number);
        } else if ! maybe_enemies_counter.is_none() {
            text.sections[0].value = format!("{}{}", config.window.enemies_counter_text, counters.enemies_killed);
        } else if ! maybe_points_counter.is_none() {
            text.sections[0].value = format!("{}{}", config.window.points_counter_text, counters.points);
        }
    }
}
