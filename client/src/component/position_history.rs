#![allow(dead_code)]
use bevy::{math::Vec2, prelude::Component, reflect::Reflect};

#[derive(Component, Reflect, Debug)]
pub struct PositionHistory {
    positions: Vec<Option<Vec2>>,
    next: usize,
}

impl Default for PositionHistory {
    fn default() -> Self {
        PositionHistory::new(1)
    }
}

impl PositionHistory {
    pub fn new(capacity: usize) -> Self {
        if capacity < 1 {
            panic!("capacity can't be less than 1");
        }
        let mut positions = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            positions.push(None);
        }
        Self { positions, next: 0 }
    }

    pub fn push(&mut self, pos: Vec2) {
        let last_index = self.positions.capacity() - 1;

        let next_index = if self.next > last_index { 0 } else { self.next };

        self.next = next_index + 1;
        self.positions[next_index] = Some(pos);
    }

    pub fn get_current(&self) -> Option<&Vec2> {
        let current = if self.next == 0 {
            self.positions.capacity() - 1
        } else {
            self.next - 1
        };
        if let Some(pos) = self.positions.get(current).unwrap() {
            Some(pos)
        } else {
            None
        }
    }

    pub fn get_prev_nth(&self, n: usize) -> Option<&Vec2> {
        if n >= self.positions.capacity() {
            return None;
        }

        let current_index = if self.next == 0 {
            self.positions.capacity() - 1
        } else {
            self.next - 1
        };

        let mut index = (current_index as i32) - (n as i32);
        if index < 0 {
            index += self.positions.capacity() as i32;
        }
        if index < 0 {
            None
        } else if let Some(pos) = self.positions.get(index as usize).unwrap() {
            Some(pos)
        } else {
            None
        }
    }

    pub fn get_last(&self) -> Option<&Vec2> {
        let oldest_index = if self.positions.contains(&None) {
            0
        } else {
            let last_index = self.positions.capacity() - 1;

            if self.next > last_index {
                0
            } else {
                self.next
            }
        };

        if let Some(pos) = self.positions.get(oldest_index).unwrap() {
            Some(pos)
        } else {
            None
        }
    }
}
