use bevy::{
    math::Vec2,
    prelude::{Parent, Query, With},
    sprite::Sprite,
};

use crate::component::{Health, HealthBar};

pub fn update_health_bar(
    health_q: Query<&Health>,
    mut health_bar_q: Query<(&Parent, &mut Sprite), With<HealthBar>>,
) {
    for (parent, mut sprite) in health_bar_q.iter_mut() {
        if let Ok(parent_health) = health_q.get(parent.0) {
            let current_health_percentage =
                (parent_health.current as f32) / (parent_health.max as f32);
            sprite.custom_size = Some(Vec2::new(current_health_percentage * 20., 3.));
        }
    }
}
