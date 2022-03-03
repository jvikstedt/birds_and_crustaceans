use crate::{
    component::{Enemy, EnemyType},
    resource::AudioHandles,
};
use bevy::prelude::{Entity, EventReader, Query, Res, ResMut};
use bevy_kira_audio::Audio;

use crate::{
    component::{EntityType, Health, Hit, Player, ScoreReward, Training},
    event::CollisionEvent,
    resource::{FrameInfo, Scores},
};

pub fn handle_hits(
    mut collision_ev: EventReader<CollisionEvent>,
    frame_info: Res<FrameInfo>,
    mut health_q: Query<(
        Entity,
        &mut Health,
        Option<&Enemy>,
        Option<&ScoreReward>,
        Option<&Training>,
    )>,
    melee_hit_q: Query<(Entity, &Hit)>,
    mut scores: ResMut<Scores>,
    mut players_q: Query<(Entity, &mut Player)>,
    audio: Res<Audio>,
    audio_handles: Res<AudioHandles>,
) {
    if frame_info.confirmed {
        for c in collision_ev.iter() {
            if let EntityType::Hit = c.source_type {
                let (_, hit) = melee_hit_q.get(c.source).unwrap();

                if hit.parent == c.target {
                    continue;
                }

                if let Ok((_, mut health, enemy, score_reward, training)) =
                    health_q.get_mut(c.target)
                {
                    health.current -= hit.damage;

                    if !frame_info.disable_sound {
                        audio.play(audio_handles.get_audio_handle("click").unwrap());
                    }

                    if health.current <= 0 {
                        if let Ok(mut parent_player) = players_q.get_mut(hit.parent) {
                            if let Some(score_reward) = score_reward {
                                scores.add_score(parent_player.1.handle, score_reward.score);
                            }

                            if let Some(enemy) = enemy {
                                match enemy.enemy_type {
                                    EnemyType::Crustacean => {
                                        if !frame_info.disable_sound {
                                            audio.play(
                                                audio_handles.get_audio_handle("frog").unwrap(),
                                            );
                                        }
                                    }
                                    EnemyType::Bird => {
                                        if !frame_info.disable_sound {
                                            audio.play(
                                                audio_handles.get_audio_handle("bird").unwrap(),
                                            );
                                        }
                                    }
                                }
                            }

                            if let Some(training) = training {
                                parent_player.1.damage += training.damage_increase;
                                parent_player.1.area += training.area_increase;

                                if parent_player.1.damage > 30 {
                                    parent_player.1.damage = 30;
                                } else if !frame_info.disable_sound {
                                    audio.play(audio_handles.get_audio_handle("powerup").unwrap());
                                }
                                if parent_player.1.area > 60 {
                                    parent_player.1.area = 60;
                                } else if !frame_info.disable_sound {
                                    audio.play(audio_handles.get_audio_handle("powerup").unwrap());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
