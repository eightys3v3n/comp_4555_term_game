use bevy::{
    prelude::*,
    input::{
        keyboard::KeyboardInput,
        ButtonState,
    },
};
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

                                fire_bullet_events.send(FireBulletEvent{
                                    bullet_type: current_weapon.bullet_type,
                                    start_transform: *player_transform,
                                });
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
                    }
                    None => {}
                }
            },
            ButtonState::Released => {}
        }
    }
}
