use bevy::{
    prelude::*,
};
use log::warn;
use super::super::{
    component::*,
    enums::*,
    event::*,
    resource::config::{ Config, BulletConfig },
};


pub fn fire_bullet(
    mut commands: Commands,
    mut fire_bullet_events: EventReader<FireBulletEvent>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    audio: Res<Audio>
) {
    for event in fire_bullet_events.iter() {
        let bullet_config: &BulletConfig;

        bullet_config = match event.bullet_type {
            BulletType::Basic => &config.bullet.basic,
            _ => panic!("Unimplemented bullet type {:?}", event.bullet_type),
        };

        let mut bullet_velocity = Vec2::from_angle(event.start_transform.rotation.to_scaled_axis().z);
        bullet_velocity *= Vec2::new(bullet_config.speed, bullet_config.speed);

        // Fixed precision to avoid rounding errors resulting in extremely tiny, non-zero values.
        bullet_velocity.x = (bullet_velocity.x * 10000.).round() / 10000.;
        bullet_velocity.y = (bullet_velocity.y * 10000.).round() / 10000.;

        let mut transform = event.start_transform;
        transform.rotation = Quat::from_rotation_z((event.start_transform.rotation.to_scaled_axis().z.to_degrees() + 90.).to_radians());

        commands.spawn(
            BulletBundle {
                sprite: SpriteBundle {
                    texture: asset_server.load(&bullet_config.image_path),
                    transform: transform,
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(bullet_config.width, bullet_config.height)),
                        ..default()
                    },
                    ..default()
                },
                velocity: Velocity::new(bullet_config.speed, event.start_transform.rotation.to_scaled_axis().z.to_degrees()),
                bullet_info: BulletInfo {
                    r#type: event.bullet_type,
                    range: bullet_config.range,
                    start_transform: event.start_transform,
                    damage: bullet_config.damage,
                    hit_something: false,
                },
                collide_info: CollideInfo {
                    radius: bullet_config.size,
                    entity_type: EntityType::Bullet,
                },
            },
        );

        audio.play(asset_server.load(&config.sound.shoot_basic_path));

        info!("Firing bullet {:?} with velocity {:?} and direction {:?}", event.bullet_type, bullet_velocity, event.start_transform.rotation.to_scaled_axis().z.to_degrees());
    }
}

pub fn despawner(
    mut commands: Commands,
    config: Res<Config>,
    bullets_query: Query<(Entity, &BulletInfo, &Transform)>,
){
    for (entity, bullet_info, transform) in bullets_query.iter() {
        if bullet_info.hit_something {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        if transform.translation.distance(bullet_info.start_transform.translation) >= bullet_info.range {
            commands.entity(entity).despawn_recursive();
            continue;
        }
    }
}
