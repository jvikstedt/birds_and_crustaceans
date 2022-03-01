use bevy::prelude::{Query, Res, With, Without};

use crate::{
    component::{Checksum, Collider, Dynamic, Fixed, NetworkEntity},
    resource::FrameInfo,
};

pub fn calculate_checksum(
    frame_info: Res<FrameInfo>,
    mut dynamic_q: Query<
        (&NetworkEntity, &Collider, &mut Checksum),
        (With<Dynamic>, Without<Fixed>),
    >,
) {
    if frame_info.confirmed {
        for (network_entity, collider, mut checksum) in dynamic_q.iter_mut() {
            let network_id = network_entity.id as f64;

            checksum.value = network_id
                + (collider.pos.x as f64)
                + (collider.pos.y as f64)
                + (collider.size.x as f64)
                + (collider.size.y as f64)
                + (collider.vel.x as f64)
                + (collider.vel.y as f64);
        }
    }
}
