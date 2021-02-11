use std::ops::Range;

pub struct WyRand {
    seed: u64,
}

impl WyRand {
    fn seed_from_u64(seed: u64) -> Self {
        WyRand { seed }
    }
    
    fn next_u64(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(0xa0761d6478bd642f);
        let t: u128 = (self.seed as u128).wrapping_mul((self.seed ^ 0xe7037ed1a0b428db) as u128);
        let ret = ((t >> 64) ^ t) as u64;
        ret
    }

    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

/// PRNG implementation using PCG-XSH-RR algorithm
pub struct Pcg32 {
    state: u64,
    // inc: u64,
}

impl Pcg32 {
    const INCREMENT: u64 = 1442695040888963407;
    const MULTIPLIER: u64 = 6364136223846793005;

    fn seed_from_u64(seed: u64) -> Self {
        let mut rng = Self {
            state: 0,
            // inc: increment.wrapping_shl(1) | 1,
        };
        let _ = rng.next_u32();
        rng.state = rng.state.wrapping_add(seed);
        let _ = rng.next_u32();
        rng
    }

    fn step(&mut self) {
        self.state = self.state
            .wrapping_mul(Self::MULTIPLIER)
            .wrapping_add(Self::INCREMENT);
    }

    fn next_u32(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        self.step();
        let xorshifted: u32 = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot: u32 = (oldstate >> 59) as u32;
        xorshifted.rotate_right(rot)
    }
}

pub struct PathSample(Pcg32);

impl PathSample {
    pub fn from_seed(seed: u64) -> Self {
        PathSample(Pcg32::seed_from_u64(seed))
    }

    pub fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }
    
    /// Returns a pair of f32s in the half-open range [0, 1)
    pub fn next_2d(&mut self) -> [f32; 2] {
        let u1 = u32_to_uniform_f32(self.0.next_u32());
        let u2 = u32_to_uniform_f32(self.0.next_u32());
        [u1, u2]
    }

    /// Returns a number in the half-open range [0, 1)
    pub fn next_f32(&mut self) -> f32 {
        u32_to_uniform_f32(self.0.next_u32())
    }

    /// Returns a random integer in the given range.
    /// Be aware that this is slightly biased
    pub fn next_range(&mut self, range: Range<u32>) -> u32 {
        // https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
        let span = range.end - range.start;
        let u = self.next_u32();
        let i = ((span as u64) * (u as u64)) >> 32;
        range.start + (i as u32)
    }
}

fn u32_to_uniform_f32(i: u32) -> f32 {
    let i = i >> 8;
    // 0.000000059604645 == 1.0/16777216.0 == (1 / 2^24)
    (i as f32) * (0.000000059604645)
}

#[test]
fn test_in_ranges() {
    let mut r = PathSample::from_seed(353849752);
    for _ in 0..100_000 {
        let a = r.next_range(0..256);
        assert!(a < 256);
        let a = r.next_range(100..20_000);
        assert!(a < 20_000 && a >= 100);
    }
    for _ in 0..100_000 {
        let f = r.next_f32();
        assert!(f >= 0.0 && f < 1.0);
    }
}