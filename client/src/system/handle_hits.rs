use bevy::prelude::{Entity, EventReader, Query, Res};

use crate::{
    component::{EntityType, Health, Hit},
    event::CollisionEvent,
    resource::FrameInfo,
};

pub fn handle_hits(
    mut collision_ev: EventReader<CollisionEvent>,
    frame_info: Res<FrameInfo>,
    mut health_q: Query<(Entity, &mut Health)>,
    melee_hit_q: Query<(Entity, &Hit)>,
) {
    if frame_info.confirmed {
        for c in collision_ev.iter() {
            if let EntityType::Hit = c.source_type {
                let (_, melee_hit) = melee_hit_q.get(c.source).unwrap();

                if melee_hit.parent == c.target {
                    continue;
                }

                if let Ok((_, mut health)) = health_q.get_mut(c.target) {
                    health.current -= melee_hit.damage;
                }
            }
        }
    }
}
