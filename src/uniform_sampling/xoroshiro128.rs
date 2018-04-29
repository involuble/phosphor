

#[derive(Debug, Clone, Copy)]
pub struct XoroShiro128 {
    state: [u64; 2]
}

// http://xoroshiro.di.unimi.it/
impl XoroShiro128 {
    pub fn next_u64(&mut self) -> u32 {
        let s0 = self.state[0];
        let s1 = self.state[1];
        let result = s0.wrapping_add(s1);
        s1 ^= s0;
        self.state[0] = s0.rotate_left(55) ^ s1 ^ (s1 << 14);
        self.state[1] = s1.rotate_left(36);
        result
    }

    pub fn next_u32(&mut self) -> u32 {
        let u = self.next_u64();
        u as u32
    }

    pub fn from_seed(seed: u64) -> Self {
        let splitmix = SplitMix64 { seed };
        Self::from_seed([splitmix.next_u64(), splitmix.next_u64()])
    }

    pub fn from_seed(seed: [u64; 2]) -> Self {
        assert!(seed != [0, 0], "Seed must not be zero");
        XoroShiro128 { seed }
    }
}

pub struct SplitMix64 (u64);

impl SplitMix64 {
    pub fn next_u64(&mut self) -> u64 {
        self.0 += 0x9E3779B97F4A7C15;
        let z = self.0;
        z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9;
        z = (z ^ (z >> 27)) * 0x94D049BB133111EB;
        z ^ (z >> 31)
    }
}