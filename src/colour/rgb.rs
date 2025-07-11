use std::ops::*;
use std::fmt;
use std::marker::PhantomData;

pub trait ColourSpace {
    const NAME: &'static str;
    const XYZ_TO_RGB: [f32; 9];
}

// Colour space reference: http://www.brucelindbloom.com/index.html?Math.html

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
    // TODO: Add per-channel weights to calculate luminance?
}

/// RGB colour vector in a linear colour space
#[repr(C)]
#[derive(PartialEq)]
pub struct Rgb<S: ColourSpace> {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    phantom: PhantomData<S>,
}

pub type Colour = Rgb<Rec709>;

impl<S: ColourSpace> Rgb<S> {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Rgb {
            r: r,
            g: g,
            b: b,
            phantom: PhantomData,
        }
    }
    
    pub fn grey(value: f32) -> Self {
        Self::new(value, value, value)
    }

    pub fn splat(value: f32) -> Self {
        Self::new(value, value, value)
    }

    pub fn is_nan(&self) -> bool {
        self.r.is_nan() | self.g.is_nan() | self.b.is_nan()
    }

    pub fn one() -> Self {
        Rgb::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Rgb::new(0.0, 0.0, 0.0)
    }

    pub fn is_zero(&self) -> bool {
        self.r == 0.0 && self.g == 0.0 && self.b == 0.0
    }

    // pub fn black() -> Self {
    //     Self::zero()
    // }
}

// FIXME: These should be deriveable but aren't yet because of PhantomData
impl<S: ColourSpace> Copy for Rgb<S> {}
impl<S: ColourSpace> Clone for Rgb<S> {
    fn clone(&self) -> Self {
        Rgb::new(self.r, self.g, self.b)
    }
}

impl<S: ColourSpace> fmt::Debug for Rgb<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.r)
            .field(&self.g)
            .field(&self.b)
            .finish()
        //write!(f, "({:?},{:?},{:?})", self.r, self.g, self.b)
    }
}

impl<S: ColourSpace> fmt::Display for Rgb<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "({},{},{})", self.r, self.g, self.b)
        write!(f, "(")?;
        fmt::Display::fmt(&self.r, f)?;
        write!(f, ", ")?;
        fmt::Display::fmt(&self.g, f)?;
        write!(f, ", ")?;
        fmt::Display::fmt(&self.b, f)?;
        write!(f, ")")
    }
}

impl<S: ColourSpace> From<[f32; 3]> for Rgb<S> {
    fn from(a: [f32; 3]) -> Self {
        Rgb::new(a[0], a[1], a[2])
    }
}

impl<S: ColourSpace> Mul<Rgb<S>> for Rgb<S> {
    type Output = Rgb<S>;

    fn mul(self, rhs: Rgb<S>) -> Self::Output {
        Rgb::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl<S: ColourSpace> Mul<Rgb<S>> for f32 {
    type Output = Rgb<S>;

    fn mul(self, rhs: Rgb<S>) -> Self::Output {
        Rgb::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl<S: ColourSpace> MulAssign<Rgb<S>> for Rgb<S> {
    fn mul_assign(&mut self, rhs: Rgb<S>) {
        *self = *self * rhs;
    }
}

impl<S: ColourSpace> Add<Rgb<S>> for Rgb<S> {
    type Output = Rgb<S>;

    fn add(self, rhs: Rgb<S>) -> Self::Output {
        Rgb::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl<S: ColourSpace> Sub<Rgb<S>> for Rgb<S> {
    type Output = Rgb<S>;

    fn sub(self, rhs: Rgb<S>) -> Self::Output {
        Rgb::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl<S: ColourSpace> AddAssign<Rgb<S>> for Rgb<S> {
    fn add_assign(&mut self, rhs: Rgb<S>) {
        *self = *self + rhs;
    }
}

impl<S: ColourSpace> Mul<f32> for Rgb<S> {
    type Output = Rgb<S>;

    fn mul(self, rhs: f32) -> Self::Output {
        Rgb::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl<S: ColourSpace> Div<f32> for Rgb<S> {
    type Output = Rgb<S>;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Rgb::new(self.r * inv, self.g * inv, self.b * inv)
    }
}
