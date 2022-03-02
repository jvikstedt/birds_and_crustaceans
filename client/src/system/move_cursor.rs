use bevy::{
    prelude::{Query, Res, Transform, With},
    sprite::Sprite,
};
use glam::Vec3;

use crate::{
    component::{Collider, MyCursor, Myself},
    resource::MouseInfo,
};

pub fn move_cursor(
    mut my_cursor_q: Query<(&mut Sprite, &mut Transform), With<MyCursor>>,
    player_q: Query<&Collider, With<Myself>>,
    mouse_info: Res<MouseInfo>,
) {
    let my_cursor = my_cursor_q.get_single_mut();
    let my_collider = player_q.get_single();

    if let Ok(mut cursor) = my_cursor {
        if let Ok(collider) = my_collider {
            cursor.0.custom_size = Some(collider.size);
            cursor.1.translation = Vec3::new(mouse_info.x as f32, mouse_info.y as f32, 999.);
        }
    }
}
