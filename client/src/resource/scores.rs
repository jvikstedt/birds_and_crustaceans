#![allow(dead_code)]
use shared::PlayerHandle;
use std::collections::HashMap;

pub type ScoreStat = i32;

pub struct Scores {
    pub scores: HashMap<PlayerHandle, ScoreStat>,
}

impl Scores {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.scores = HashMap::new();
    }

    pub fn add_score(&mut self, handle: PlayerHandle, score: ScoreStat) {
        *self.scores.entry(handle).or_insert(0) += score;
    }

    pub fn get_score(&mut self, handle: PlayerHandle) -> ScoreStat {
        self.scores.get(&handle).cloned().unwrap_or(0)
    }

    pub fn get_scores(&self) -> Vec<(&PlayerHandle, &ScoreStat)> {
        Vec::from_iter(self.scores.iter())
    }
}
