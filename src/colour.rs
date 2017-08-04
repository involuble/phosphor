use num_traits::{Float, Zero, clamp, NumCast};
use std::ops::{Add, Mul, Div, MulAssign, AddAssign};
use approx::ApproxEq;

#[derive(Debug, Clone, Copy)]
pub struct ColourBase<T> where T: Float {
    pub r: T,
    pub g: T,
    pub b: T,
}

#[allow(dead_code)]
// Temp should be roughly in range 1000K..40000K
fn blackbody_to_colour(temp: f32) -> Colour {
    // See http://www.zombieprototypes.com/?p=210
    //  & http://www.vendian.org/mncharity/dir3/blackbody/UnstableURLs/bbr_color.html
    let temp = clamp(temp, 1000.0, 40000.0);
    let red;
    if temp >= 6600.0 {
        let x = temp/100.0 - 55.0;
        red = 351.976905668 + 0.114206453784*x + -40.2536630933*x.ln();
    } else {
        red = 255.0;
    }

    let green;
    if temp < 6600.0 {
        let x = temp/100.0 - 2.0;
        green = -155.254855627 + -0.445969504695*x + 104.492161993*x.ln();
    } else {
        // if temp >= 6600
        let x = temp/100.0 - 50.0;
        green = 325.449412571 + 0.0794345653666*x + -28.0852963507*x.ln();
    }

    let blue;
    if temp <= 2000.0 {
        blue = 0.0;
    } else if temp < 6600.0 {
        let x = temp/100.0 - 10.0;
        blue = -254.769351841 + 0.827409606400*x + 115.679944010*x.ln();
    } else {
        blue = 255.0;
    }
    Colour::new(red/255.0, green/255.0, blue/255.0)
}

impl<T> ColourBase<T> where T: Float {
    pub fn from_luma(y: T) -> Self {
        ColourBase { r: y, g: y, b: y }
    }

    pub fn new(r: T, g: T, b: T) -> Self {
        ColourBase { r: r, g: g, b: b }
    }

    pub fn black() -> Self {
        ColourBase::zero()
    }

    pub fn zero() -> Self {
        ColourBase { r: Zero::zero(), g: Zero::zero(), b: Zero::zero() }
    }

    pub fn is_black(&self) -> bool {
        self.r == T::zero() && self.g == T::zero() && self.b == T::zero()
    }

    // TODO: move from_srgb & to_srgb into a trait
    #[allow(dead_code)]
    fn from_srgb(b: u8) -> T {
        let c = b as f32 / 255.0;
        let c_linear = if c <= 0.04045 {
            c / 12.92
        } else {
            let a: f32 = 0.055;
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
            let a: f32 = 0.055;
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

impl<T> Div<T> for ColourBase<T> where T: Float {
    type Output = ColourBase<T>;

    fn div(self, rhs: T) -> Self::Output {
        ColourBase { r: self.r / rhs, g: self.g / rhs, b: self.b / rhs }
    }
}

impl<T> MulAssign<ColourBase<T>> for ColourBase<T> where T: Float {
    fn mul_assign(&mut self, rhs: ColourBase<T>) {
        *self = *self * rhs;
    }
}

impl<T> AddAssign<ColourBase<T>> for ColourBase<T> where T: Float {
    fn add_assign(&mut self, rhs: ColourBase<T>) {
        *self = *self + rhs;
    }
}

impl<T: ApproxEq> ApproxEq for ColourBase<T> where T: Float, T::Epsilon: Copy {
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> T::Epsilon {
        T::default_epsilon()
    }

    fn default_max_relative() -> T::Epsilon {
        T::default_max_relative()
    }

    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
        T::relative_eq(&self.r, &other.r, epsilon, max_relative) &&
        T::relative_eq(&self.g, &other.g, epsilon, max_relative) &&
        T::relative_eq(&self.b, &other.b, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.r, &other.r, epsilon, max_ulps) &&
        T::ulps_eq(&self.g, &other.g, epsilon, max_ulps) &&
        T::ulps_eq(&self.b, &other.b, epsilon, max_ulps)
    }
}

pub type Colour = ColourBase<f32>;