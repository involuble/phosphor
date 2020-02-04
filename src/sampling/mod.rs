pub use rand::{Rng};
pub use rand_pcg::Pcg32;

pub type SampleRng = Pcg32;

fn u32_to_uniform_f32(i: u32) -> f32 {
    let i = i >> 8;
    // 0.000000059604645 == 1.0/16777216.0 == (1 / 2^24)
    (i as f32) * (0.000000059604645)
}