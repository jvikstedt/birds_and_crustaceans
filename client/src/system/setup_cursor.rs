use bevy::{
    prelude::{AssetServer, Commands, Res, Transform},
    sprite::{Sprite, SpriteBundle},
};
use glam::Vec2;

use crate::component::MyCursor;

pub fn setup_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(MyCursor {})
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("cursor.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(0., 0.)),
                ..Default::default()
            },

            transform: Transform {
                translation: [0., 0., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        });
}
