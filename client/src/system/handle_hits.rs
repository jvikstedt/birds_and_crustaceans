use bevy::prelude::{Entity, EventReader, Query, Res, ResMut};

use crate::{
    component::{EntityType, Health, Hit, Player},
    event::CollisionEvent,
    resource::{FrameInfo, Scores},
};

pub fn handle_hits(
    mut collision_ev: EventReader<CollisionEvent>,
    frame_info: Res<FrameInfo>,
    mut health_q: Query<(Entity, &mut Health)>,
    melee_hit_q: Query<(Entity, &Hit)>,
    mut scores: ResMut<Scores>,
    players_q: Query<(Entity, &Player)>,
) {
    if frame_info.confirmed {
        for c in collision_ev.iter() {
            if let EntityType::Hit = c.source_type {
                let (_, hit) = melee_hit_q.get(c.source).unwrap();

                if hit.parent == c.target {
                    continue;
                }

                if let Ok((_, mut health)) = health_q.get_mut(c.target) {
                    health.current -= hit.damage;

                    if health.current <= 0 {
                        if let Ok(parent_player) = players_q.get(hit.parent) {
                            scores.add_score(parent_player.1.handle, 10);
                        }
                    }
                }
            }
        }
    }
}
