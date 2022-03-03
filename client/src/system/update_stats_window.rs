use bevy::{
    prelude::{Query, With},
    text::Text,
};

use crate::component::{Myself, Player, StatsWindow};

pub fn update_stats_window(
    mut text_q: Query<&mut Text, With<StatsWindow>>,
    player_q: Query<&Player, With<Myself>>,
) {
    if let Ok(mut text) = text_q.get_single_mut() {
        if let Ok(player) = player_q.get_single() {
            let mut str: String = String::new();
            str.push_str(&format!("\nDamage: {}/30", player.damage));
            str.push_str(&format!("\nArea: {}/60", player.area));
            text.sections[1].value = str;
        }
    }
}
