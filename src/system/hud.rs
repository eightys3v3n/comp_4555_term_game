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
        store::Store,
    },
};


pub fn updater(
    mut query: Query<(&mut Text, &UpdatableTextField), With<HUD>>,
    round_info: Res<RoundInfo>,
    config: Res<Config>,
    asset_server: Res<AssetServer>,
    counters: Res<Counters>,
    store: Res<Store>,
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
            TextField::DamageModifier => {
                let damage_modifier = config.store.modifiers.get(&Modifier::Damage).unwrap();
                let damage_cost = damage_modifier.cost * 2_f32.powi(store.purchase_count_damage);
                text.sections[1].value = format!("${}", damage_cost);
            }
            _ => {},
        };
    }
}
