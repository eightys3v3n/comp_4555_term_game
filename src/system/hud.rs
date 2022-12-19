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
    player_health_query: Query<&Health, With<Player>>,
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
            TextField::RangeModifier => {
                let range_modifier = config.store.modifiers.get(&Modifier::Range).unwrap();
                let range_cost = range_modifier.cost * 2_f32.powi(store.purchase_count_range);
                text.sections[1].value = format!("${}", range_cost);
            },
            TextField::FireRateModifier => {
                let fire_rate_modifier = config.store.modifiers.get(&Modifier::FireRate).unwrap();
                let fire_rate_cost = fire_rate_modifier.cost * 2_f32.powi(store.purchase_count_fire_rate);
                text.sections[1].value = format!("${}", fire_rate_cost);
            },
            TextField::Heal => {
                let heal_modifier = config.store.modifiers.get(&Modifier::Heal).unwrap();
                let heal_cost = heal_modifier.cost * 2_f32.powi(store.purchase_count_heal);
                text.sections[1].value = format!("${}", heal_cost);
            },
            TextField::Health => {
                if player_health_query.is_empty() {
                    warn!("Can't find the player health object to update the HUD");
                    continue;
                }
                let player_health = player_health_query.get_single().unwrap();

                text.sections[1].value = format!("{}/{}", player_health.current, player_health.max);
            },
            _ => {},
        };
    }
}
