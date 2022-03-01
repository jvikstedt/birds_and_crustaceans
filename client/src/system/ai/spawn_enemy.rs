use bevy::{
    math::{Vec2, Vec3},
    prelude::{AssetServer, BuildChildren, Color, Commands, Res, ResMut, Transform},
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    component::{ai, Checksum, Collider, Dynamic, EntityType, Health, HealthBar, NetworkEntity},
    resource::NetworkIdProvider,
    rollback::{Rollback, RollbackIdProvider},
};

pub fn spawn_enemy(
    mut commands: Commands,
    mut nip: ResMut<NetworkIdProvider>,
    mut rip: ResMut<RollbackIdProvider>,
    asset_server: Res<AssetServer>,
) {
    let size_x = 40.;
    let size_y = 26.;

    let health_bar_id = commands
        .spawn()
        .insert(HealthBar::default())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0.0, 0.0),
                custom_size: Some(Vec2::new(20., 3.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., size_x / 2. + 10., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("crustacean.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(size_x, size_y)),
                ..Default::default()
            },

            transform: Transform {
                translation: [100., 200., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(size_x, size_y),
            pos: Vec2::new(100., 200.),
            vel: Vec2::default(),
        })
        .insert(Dynamic::default())
        .insert(EntityType::Enemy)
        .insert(NetworkEntity { id: nip.next_id() })
        .insert(Rollback::new(rip.next_id()))
        .insert(Checksum::default())
        .insert(ai::Smart::default())
        .insert(Health::new(50, 50))
        .insert(ai::actions::Wander)
        .add_child(health_bar_id);

    let size_x = 40.;
    let size_y = 37.6;

    let health_bar_id = commands
        .spawn()
        .insert(HealthBar::default())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0.0, 0.0),
                custom_size: Some(Vec2::new(20., 3.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., size_x / 2. + 10., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("bird.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(size_x, size_y)),
                ..Default::default()
            },

            transform: Transform {
                translation: [300., 200., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider {
            size: Vec2::new(size_x, size_y),
            pos: Vec2::new(300., 200.),
            vel: Vec2::default(),
        })
        .insert(Dynamic::default())
        .insert(EntityType::Enemy)
        .insert(NetworkEntity { id: nip.next_id() })
        .insert(Rollback::new(rip.next_id()))
        .insert(Checksum::default())
        .insert(ai::Smart::default())
        .insert(Health::new(100, 100))
        .insert(ai::actions::Wander)
        .add_child(health_bar_id);
}
