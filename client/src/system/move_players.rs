use bevy::{
    math::Vec3,
    prelude::{Query, Res, Transform, With},
};

use crate::{
    component::{Collider, Dynamic, PositionHistory},
    resource::RenderInfo,
};

// Move actual player entities Transform component positions to match the collider's position
// This does not need to be executed deterministically
pub fn move_players(
    mut player_query: Query<(&mut Transform, &Collider, Option<&PositionHistory>), With<Dynamic>>,
    render_info: Res<RenderInfo>,
) {
    for (mut transform, collider, position_his) in player_query.iter_mut() {
        if let Some(position_his) = position_his {
            if let Some(pos) = position_his.get_last() {
                transform.translation = transform.translation.lerp(
                    Vec3::new(pos.x, pos.y, transform.translation.z),
                    render_info.delta,
                );
                continue;
            }
        }

        transform.translation = transform.translation.lerp(
            Vec3::new(collider.pos.x, collider.pos.y, transform.translation.z),
            render_info.delta,
        );
    }
}
