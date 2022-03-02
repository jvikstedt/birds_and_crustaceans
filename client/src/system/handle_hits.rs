use bevy::prelude::{Entity, EventReader, Query, Res, ResMut};

use crate::{
    component::{EntityType, Health, Hit, Player, ScoreReward, Training},
    event::CollisionEvent,
    resource::{FrameInfo, Scores},
};

pub fn handle_hits(
    mut collision_ev: EventReader<CollisionEvent>,
    frame_info: Res<FrameInfo>,
    mut health_q: Query<(Entity, &mut Health, Option<&ScoreReward>, Option<&Training>)>,
    melee_hit_q: Query<(Entity, &Hit)>,
    mut scores: ResMut<Scores>,
    mut players_q: Query<(Entity, &mut Player)>,
) {
    if frame_info.confirmed {
        for c in collision_ev.iter() {
            if let EntityType::Hit = c.source_type {
                let (_, hit) = melee_hit_q.get(c.source).unwrap();

                if hit.parent == c.target {
                    continue;
                }

                if let Ok((_, mut health, score_reward, training)) = health_q.get_mut(c.target) {
                    health.current -= hit.damage;

                    if health.current <= 0 {
                        if let Ok(mut parent_player) = players_q.get_mut(hit.parent) {
                            if let Some(score_reward) = score_reward {
                                scores.add_score(parent_player.1.handle, score_reward.score);
                            }

                            if let Some(training) = training {
                                parent_player.1.damage += training.damage_increase;
                                parent_player.1.area += training.area_increase;

                                if parent_player.1.damage > 30 {
                                    parent_player.1.damage = 30;
                                }
                                if parent_player.1.area > 60 {
                                    parent_player.1.area = 60;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
