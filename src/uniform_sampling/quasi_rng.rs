

pub trait QRNG {
    pub fn next(&mut self) -> f32;
}

fn f32_from_u32(u: u32) -> f32 {
    const UPPER_MASK: u32 = 0x3F800000;
    const LOWER_MASK: u32 = 0x007FFFFF;
    let result = UPPER_MASK | (u & LOWER_MASK);
    f32::from_bits(result) - 1.0
}
