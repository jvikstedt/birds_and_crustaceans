use bevy::{
    math::Vec2,
    prelude::{Color, Commands, ResMut, SpriteBundle, Transform},
    sprite::Sprite,
};

use crate::{
    component::{Collider, Fixed, NetworkEntity},
    resource::NetworkIdProvider,
};

pub fn build_map(mut commands: Commands, mut nip: ResMut<NetworkIdProvider>) {
    let tile_size = 100.;

    let mut entity_commands = commands.spawn();
    entity_commands.insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.5, 0.5, 1.0),
            custom_size: Some(Vec2::new(tile_size, tile_size)),
            ..Default::default()
        },

        transform: Transform {
            translation: [100., 100., 0.].into(),
            ..Default::default()
        },
        ..Default::default()
    });

    entity_commands.insert(Collider {
        size: Vec2::new(tile_size, tile_size),
        pos: Vec2::new(100., 100.),
        vel: Vec2::default(),
    });
    entity_commands.insert(Fixed::default());
    entity_commands.insert(NetworkEntity { id: nip.next_id() });
}
