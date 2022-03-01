use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct FrameDiff {
    track: VecDeque<i32>,
    keep: usize,
}

impl Default for FrameDiff {
    fn default() -> Self {
        Self {
            track: VecDeque::new(),
            keep: 10,
        }
    }
}

impl FrameDiff {
    pub fn new(keep: usize) -> Self {
        Self {
            track: VecDeque::new(),
            keep,
        }
    }

    pub fn add(&mut self, frame_diff: i32) {
        if self.track.len() >= self.keep {
            self.track.pop_front();
        }

        self.track.push_back(frame_diff);
    }

    pub fn real_average_i8(&self) -> i8 {
        let track_size = self.track.len();
        if track_size == 0 {
            return 0;
        }

        let sum: i32 = self.track.iter().sum();

        self.i32_to_i8(sum / track_size as i32)
    }

    pub fn average_i8(&self) -> i8 {
        let min = *self.track.iter().min().unwrap_or(&0);
        let max = *self.track.iter().max().unwrap_or(&0);
        let mut min_ignored = false;
        let mut max_ignored = false;

        let mut sum: i32 = 0;
        let mut count: i32 = 0;
        for diff in self.track.iter() {
            if *diff == min && !min_ignored {
                min_ignored = true;
                continue;
            }

            if *diff == max && !max_ignored {
                max_ignored = true;
                continue;
            }

            sum += diff;
            count += 1;
        }

        if count == 0 {
            return 0;
        }

        self.i32_to_i8(sum / count)
    }

    fn i32_to_i8(&self, number: i32) -> i8 {
        if number > (i8::MAX as i32) {
            i8::MAX
        } else if number < (i8::MIN as i32) {
            i8::MIN
        } else {
            number as i8
        }
    }
}
