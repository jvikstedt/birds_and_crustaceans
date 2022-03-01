#![allow(dead_code)]
use rand::{prelude::StdRng, SeedableRng};

pub struct Random {
    seed: u64,
    rng: StdRng,
    unintialized: bool,
}

impl Default for Random {
    fn default() -> Self {
        Self {
            seed: 0,
            rng: StdRng::seed_from_u64(0),
            unintialized: true,
        }
    }
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: StdRng::seed_from_u64(seed),
            unintialized: false,
        }
    }

    // Get mutable access to rng
    // Only call this deterministically (eq: confirmed frames)
    pub fn get_mut(&mut self) -> &mut StdRng {
        if self.unintialized {
            panic!("unintialized random");
        }
        &mut self.rng
    }

    pub fn reset(&mut self) {
        self.rng = StdRng::seed_from_u64(self.seed);
        self.unintialized = false;
    }

    pub fn reseed(&mut self, seed: u64) {
        self.seed = seed;
        self.reset();
    }

    pub fn is_unintialized(&self) -> bool {
        self.unintialized
    }
}
