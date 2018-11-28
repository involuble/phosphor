#![allow(non_camel_case_types)]

use std::ops::*;
use std::fmt;
use std::marker::PhantomData;

use num_traits::{Zero, One};

pub trait ColourSpace {
    const NAME: &'static str;
    const XYZ_TO_RGB: [f32; 9];
}

/// The Rec. 709 or linear sRGB colour space
pub struct Rec709 {}

impl ColourSpace for Rec709 {
    const NAME: &'static str = "Rec. 709/Linear sRGB";
    // TODO
    const XYZ_TO_RGB: [f32; 9] = [
        3.2, -1.5, -0.4,
        -0.9, 1.8, 0.04,
        0.05, -0.2, 1.05
    ];
}

/// RGB colour vector in a particular colour space
#[repr(C)]
// #[derive(Add, Mul, AddAssign, Div)] // Can't derive these because of PhantomData
pub struct RGB<S: ColourSpace> {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    phantom: PhantomData<S>,
}

pub type Colour = RGB<Rec709>;

impl<S: ColourSpace> RGB<S> {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        RGB {
            r: r,
            g: g,
            b: b,
            phantom: PhantomData,
        }
    }

    pub fn is_nan(&self) -> bool {
        self.r.is_nan() | self.g.is_nan() | self.b.is_nan()
    }

    // pub fn black() -> Self {
    //     Self::zero()
    // }
}

impl<S: ColourSpace> Copy for RGB<S> {}
impl<S: ColourSpace> Clone for RGB<S> {
    fn clone(&self) -> Self {
        RGB::new(self.r, self.g, self.b)
    }
}

impl<S: ColourSpace> fmt::Debug for RGB<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?},{:?})", self.r, self.g, self.b)
    }
}

impl<S: ColourSpace> fmt::Display for RGB<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.r, self.g, self.b)
    }
}

impl<S: ColourSpace> Zero for RGB<S> {
    fn zero() -> Self {
        RGB::new(0.0, 0.0, 0.0)
    }

    fn is_zero(&self) -> bool {
        self.r == 0.0 && self.g == 0.0 && self.b == 0.0
    }
}

impl<S: ColourSpace> One for RGB<S> {
    fn one() -> Self {
        RGB::new(1.0, 1.0, 1.0)
    }

    fn is_one(&self) -> bool {
        self.r == 1.0 && self.g == 1.0 && self.b == 1.0
    }
}

impl<S: ColourSpace> Mul<RGB<S>> for RGB<S> {
    type Output = RGB<S>;

    fn mul(self, rhs: RGB<S>) -> Self::Output {
        RGB::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl<S: ColourSpace> Mul<RGB<S>> for f32 {
    type Output = RGB<S>;

    fn mul(self, rhs: RGB<S>) -> Self::Output {
        RGB::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl<S: ColourSpace> MulAssign<RGB<S>> for RGB<S> {
    fn mul_assign(&mut self, rhs: RGB<S>) {
        *self = *self * rhs;
    }
}

// These should be derived
impl<S: ColourSpace> Add<RGB<S>> for RGB<S> {
    type Output = RGB<S>;

    fn add(self, rhs: RGB<S>) -> Self::Output {
        RGB::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl<S: ColourSpace> Sub<RGB<S>> for RGB<S> {
    type Output = RGB<S>;

    fn sub(self, rhs: RGB<S>) -> Self::Output {
        RGB::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl<S: ColourSpace> AddAssign<RGB<S>> for RGB<S> {
    fn add_assign(&mut self, rhs: RGB<S>) {
        *self = *self + rhs;
    }
}

impl<S: ColourSpace> Mul<f32> for RGB<S> {
    type Output = RGB<S>;

    fn mul(self, rhs: f32) -> Self::Output {
        RGB::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl<S: ColourSpace> Div<f32> for RGB<S> {
    type Output = RGB<S>;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        RGB::new(self.r * inv, self.g * inv, self.b * inv)
    }
}
