use bevy::prelude::{Query, Res, Transform, With};
use bevy_prototype_lyon::{
    prelude::{Path, ShapePath},
    shapes,
};
use glam::Vec3;

use crate::{
    component::{Collider, MyCursor, Myself},
    resource::MouseInfo,
};

pub fn move_cursor(
    mut my_cursor_q: Query<(&mut Transform, &mut Path), With<MyCursor>>,
    player_q: Query<&Collider, With<Myself>>,
    mouse_info: Res<MouseInfo>,
) {
    let my_cursor = my_cursor_q.get_single_mut();
    let my_collider = player_q.get_single();

    if let Ok(mut cursor) = my_cursor {
        if let Ok(collider) = my_collider {
            let shape = shapes::RegularPolygon {
                sides: 4,
                feature: shapes::RegularPolygonFeature::SideLength(collider.size.x),
                ..shapes::RegularPolygon::default()
            };
            *cursor.1 = ShapePath::build_as(&shape);
            cursor.0.translation = Vec3::new(mouse_info.x as f32, mouse_info.y as f32, 999.);
        }
    }
}
