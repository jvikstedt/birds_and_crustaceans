use bevy::{
    prelude::{Query, Res, With},
    text::Text,
};

use crate::{component::ScoreWindow, resource::Scores};

pub fn update_score_window(mut text_q: Query<&mut Text, With<ScoreWindow>>, scores: Res<Scores>) {
    let mut text = text_q.get_single_mut().unwrap();

    let mut str: String = String::new();

    let all_scores = scores.get_scores();

    for (handle, score) in all_scores {
        str.push_str(&format!("\n{}: {}", handle, score));
    }

    text.sections[1].value = str;
}
