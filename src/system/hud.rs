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
    mut query: Query<(&mut Text, &UpdatableTextField), With<HUD>>,
    round_info: Res<RoundInfo>,
    config: Res<Config>,
    asset_server: Res<AssetServer>,
    counters: Res<Counters>,
) {
    for (mut text, updatable_text) in query.iter_mut() {
        match updatable_text.field {
            TextField::RoundCounter => {
                text.sections[0].value = format!("{}{}", config.window.round_counter_text, round_info.number);
            },
            TextField::EnemiesCounter => {
                text.sections[0].value = format!("{}{}", config.window.enemies_counter_text, counters.enemies_killed);
            },
            TextField::PointsCounter => {
                text.sections[0].value = format!("{}{}", config.window.points_counter_text, counters.points);
            },
            _ => {},
        };
    }
}
