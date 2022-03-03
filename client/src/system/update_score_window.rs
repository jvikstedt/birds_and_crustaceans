use bevy::{
    prelude::{Query, Res, With},
    text::Text,
};
use shared::message::Information;

use crate::{component::ScoreWindow, resource::Scores};

pub fn update_score_window(
    mut text_q: Query<&mut Text, With<ScoreWindow>>,
    scores: Res<Scores>,
    information: Res<Information>,
) {
    if let Ok(mut text) = text_q.get_single_mut() {
        let mut str: String = String::new();

        let mut all_scores = scores.get_scores();
        all_scores.sort_by_key(|k| k.1);
        all_scores.reverse();

        for (handle, score) in all_scores {
            if information.player_handle == *handle {
                str.push_str(&format!("\n{}: {}", "You", score));
            } else {
                str.push_str(&format!("\n{}: {}", handle, score));
            }
        }

        text.sections[1].value = str;
    }
}
