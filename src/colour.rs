use num_traits::{Float, Zero};
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