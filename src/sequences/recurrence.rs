use std::default::{Default};

// https://en.wikipedia.org/wiki/Low-discrepancy_sequence#Additive_recurrence
// https://blog.demofox.org/2017/05/29/when-random-numbers-are-too-random-low-discrepancy-sequences/
pub struct AdditiveRecurrence {
    pub start: f32,
}

impl Default for AdditiveRecurrence {
    fn default() -> Self { AdditiveRecurrence { start: 0.5 }}
}

impl AdditiveRecurrence {
    pub fn from_seed(seed: f32) -> Self {
        AdditiveRecurrence { start: seed }
    }

    pub fn rand(&mut self) -> f32 {
        const BASE: f32 = 1.61803398875;
        let r = self.start;
        self.start = (self.start + BASE) % 1.0;
        r
    }
}