use bevy::{
    prelude::*,
    input::{
        keyboard::KeyboardInput,
        ButtonState,
    },
};
use std::time::{ Duration, SystemTime };
use log::warn;
use super::super::{
    enums::*,
    event::*,
    component::*,
    resource::{
        weapons::Weapons,
        config::Config,
        counter::Counters,
        store::Store,
    },
};

pub fn handle_playing_inputs(
    mut state: ResMut<State<AppState>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut fire_bullet_events: EventWriter<FireBulletEvent>,
    mut current_weapon: ResMut<Weapons>,
    mut counters: ResMut<Counters>,
    mut store: ResMut<Store>,
    config: Res<Config>,
    player_transform_query: Query<&Transform, With<Player>>,
) {
    if state.current() != &AppState::Playing {
        keyboard_events.clear();
        return;
    }

    for event in keyboard_events.iter() {
        match event.state {
            ButtonState::Pressed => {
                match event.key_code {
                    Some(key_code) => {
                        if key_code == KeyCode::Escape {
                            match state.set(AppState::MainMenu) {
                                Ok(_) => info!("Switched into Main Menu state"),
                                Err(e) => warn!("Failed to switch into the Main Menu state on Escape pressed. {}", e),
                            }
                        }
                        // else if key_code == KeyCode::Grave { // ~ symbol
                        //     match state.set(AppState::GameOver) {
                        //         Ok(_) => info!("Switched into Game Over state"),
                        //         Err(e) => warn!("Failed to switch into the Game Over state on ` pressed. {}", e),
                        //     }
                        // }
                        else if key_code == KeyCode::Space {
                            if ! player_transform_query.is_empty() {
                                let player_transform = player_transform_query.get_single().unwrap();

                                if ! current_weapon.last_fire_time.is_none() {
                                    if current_weapon.last_fire_time.unwrap().elapsed().unwrap() < Duration::from_millis((config.player.fire_delay as f32 * current_weapon.fire_rate_modifier) as u64) {
                                        continue;
                                        // can't fire because too soon.
                                    }
                                }

                                fire_bullet_events.send(FireBulletEvent{
                                    bullet_type: current_weapon.bullet_type,
                                    start_transform: *player_transform,
                                });
                                current_weapon.last_fire_time = Some(SystemTime::now());
                                // info!("Player transform: {}, {}", player_transform.translation.x, player_transform.translation.y);
                            } else {
                                warn!("Unable to fetch the player to get the direction they are facing. Can't fire bullets.");
                            }
                        }
                        else if key_code == KeyCode::U {
                            let damage_modifier = config.store.modifiers.get(&Modifier::Damage).unwrap();
                            let damage_cost = damage_modifier.cost * 2_f32.powi(store.purchase_count_damage);
                            if counters.points >= damage_cost {
                                current_weapon.damage_modifier = (current_weapon.damage_modifier * damage_modifier.amount.unwrap() * 100.0).round() / 100.0;
                                counters.points -= damage_cost;
                                info!("Upgrading damage modifier to {}.", current_weapon.damage_modifier);
                                store.purchase_count_damage += 1;
                            } else {
                                info!("Not enough points to upgrade damage modifier.");
                            }
                        }
                        else if key_code == KeyCode::I {
                            let range_modifier = config.store.modifiers.get(&Modifier::Range).unwrap();
                            let range_cost = range_modifier.cost * 2_f32.powi(store.purchase_count_range);
                            if counters.points >= range_cost {
                                current_weapon.range_modifier = (current_weapon.range_modifier * range_modifier.amount.unwrap() * 100.0).round() / 100.0;
                                counters.points -= range_cost;
                                info!("Upgrading range modifier to {}.", current_weapon.range_modifier);
                                store.purchase_count_range += 1;
                            } else {
                                info!("Not enough points to upgrade range modifier.");
                            }
                        }
                        else if key_code == KeyCode::O {
                            let fire_rate_modifier = config.store.modifiers.get(&Modifier::FireRate).unwrap();
                            let fire_rate_cost = fire_rate_modifier.cost * 2_f32.powi(store.purchase_count_fire_rate);
                            if counters.points >= fire_rate_cost {
                                current_weapon.fire_rate_modifier = (current_weapon.fire_rate_modifier * fire_rate_modifier.amount.unwrap() * 100.0).round() / 100.0;
                                counters.points -= fire_rate_cost;
                                info!("Upgrading fire_rate modifier to {}.", current_weapon.fire_rate_modifier);
                                store.purchase_count_fire_rate += 1;
                            } else {
                                info!("Not enough points to upgrade fire rate modifier.");
                            }
                        }
                    }
                    None => {}
                }
            },
            ButtonState::Released => {}
        }
    }
}
