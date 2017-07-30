use num_traits::{Float, Zero, clamp, NumCast};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct ColourBase<T> where T: Float {
    pub g: T,
    pub b: T,
    pub r: T,
}

impl<T> ColourBase<T> where T: Float {
    pub fn from_luma(y: T) -> Self {
        ColourBase { r: y, g: y, b: y }
    }

    pub fn new(r: T, g: T, b: T) -> Self {
        ColourBase { r: r, g: g, b: b }
    }

    pub fn black() -> Self {
        ColourBase { r: Zero::zero(), g: Zero::zero(), b: Zero::zero() }
    }

    // TODO: move from_srgb & to_srgb into a trait
    fn from_srgb(b: u8) -> T {
        let c = b as f32 / 255.0;
        let c_linear = if c <= 0.04045 {
            c / 12.92
        } else {
            let a = 0.055;
            Float::powf((c+a)/(1.0+a), 2.4)
        };
        NumCast::from(c_linear).unwrap()
    }

    fn to_srgb(c_linear: T) -> u8 {
        let c: f32 = clamp(NumCast::from(c_linear).unwrap(), 0.0, 1.0);
        let c_srgb;
        if c < 0.0031308 {
            c_srgb = 12.92 * c;
        } else {
            let a = 0.055;
            c_srgb = (1.0+a) * Float::powf(c, 1.0/2.4) - a;
        }
        Float::floor(c_srgb * 255.0 + 0.5) as u8
    }

    pub fn into_u8_rgb(self) -> [u8; 3] {
        [Self::to_srgb(self.r), Self::to_srgb(self.g), Self::to_srgb(self.b)]
    }
}

impl<T> Add<ColourBase<T>> for ColourBase<T> where T: Float {
    type Output = ColourBase<T>;

    fn add(self, rhs: ColourBase<T>) -> Self::Output {
        ColourBase { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
    }
}

impl<T> Mul<ColourBase<T>> for ColourBase<T> where T: Float {
    type Output = ColourBase<T>;

    fn mul(self, rhs: ColourBase<T>) -> Self::Output {
        ColourBase { r: self.r * rhs.r, g: self.g * rhs.g, b: self.b * rhs.b }
    }
}

impl<T> Mul<T> for ColourBase<T> where T: Float {
    type Output = ColourBase<T>;

    fn mul(self, rhs: T) -> Self::Output {
        ColourBase { r: self.r * rhs, g: self.g * rhs, b: self.b * rhs }
    }
}

pub type Colour = ColourBase<f32>;