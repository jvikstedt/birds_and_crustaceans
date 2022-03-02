use bevy::{prelude::Query, sprite::Sprite};

use crate::component::{Collider, Player};

pub fn update_player_size(mut player_query: Query<(&mut Sprite, &mut Collider, &Player)>) {
    for (mut sprite, mut collider, player) in player_query.iter_mut() {
        collider.size.x = player.area as f32;
        collider.size.y = player.area as f32;

        sprite.custom_size = Some(collider.size);
    }
}
