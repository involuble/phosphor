#![allow(non_camel_case_types)]

use std::ops::*;
use std::fmt::Debug;
use std::marker::PhantomData;

use num_traits::{Zero};

// The debug, copy, clone bounds are a bit of a hack
// They're needed to make derive work on RGB
pub trait ColourSpace: Debug + Clone + Copy {
    const NAME: &'static str;
    const XYZ_TO_RGB: [f32; 9];
}

#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// Can't derive these because of PhantomData
// #[derive(Add, Mul, AddAssign, Div)]
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

    // pub fn black() -> Self {
    //     Self::zero()
    // }
}

// impl<S: ColourSpace> Copy for RGB<S> {}
// impl<S: ColourSpace> Clone for RGB<S> {
//     fn clone(&self) -> Self {
//         RGB::new(self.r, self.g, self.b)
//     }
// }

impl<S: ColourSpace> Zero for RGB<S> {
    fn zero() -> Self {
        RGB::new(0.0, 0.0, 0.0)
    }

    fn is_zero(&self) -> bool {
        self.r == 0.0 && self.g == 0.0 && self.b == 0.0
    }
}

impl<S: ColourSpace> Mul<RGB<S>> for RGB<S> {
    type Output = RGB<S>;

    fn mul(self, rhs: RGB<S>) -> Self::Output {
        RGB::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
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

impl<S: ColourSpace> AddAssign<RGB<S>> for RGB<S> {
    fn add_assign(&mut self, rhs: RGB<S>) {
        *self = *self + rhs;
    }
}