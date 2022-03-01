use bevy::prelude::Query;

use crate::component::{Collider, PositionHistory};

pub fn update_position_history(mut positions_query: Query<(&mut PositionHistory, &Collider)>) {
    for (mut position_his, collider) in positions_query.iter_mut() {
        position_his.push(collider.pos);
    }
}
