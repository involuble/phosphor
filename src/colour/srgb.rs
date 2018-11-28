// #![allow(dead_code)]

use colour::rgb::*;

use num_traits::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sRgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl sRgb {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        sRgb {
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn to_rgb8(&self) -> [u8; 3] {
        [
            float_to_u8(self.r),
            float_to_u8(self.g),
            float_to_u8(self.b),
        ]
    }

    pub fn from_rgb8(rgb: [u8; 3]) -> Self {
        sRgb {
            r: u8_to_float(rgb[0]),
            g: u8_to_float(rgb[1]),
            b: u8_to_float(rgb[2]),
        }
    }
}

/// Takes a gamma corrected value in sRGB space and linearises it
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        let a: f32 = 0.055;
        Float::powf((c+a)/(1.0+a), 2.4)
    }
}

fn srgb_to_linear_approx(c_srgb: f32) -> f32 {
    c_srgb * (c_srgb * (c_srgb * 0.305306011 + 0.682171111) + 0.012522878)
}

/// Takes a value in linear colour space and gamma corrects it to sRGB space.
/// The input c is a normalised (in [0,1]) float
fn linear_to_srgb(c_linear: f32) -> f32 {
    let c = clamp(c_linear, 0.0, 1.0);
    if c < 0.0031308 {
        12.92 * c
    } else {
        let a: f32 = 0.055;
        (1.0+a) * Float::powf(c, 1.0/2.4) - a
    }
}

fn linear_to_srgb_approx(c_linear: f32) -> f32 {
    // See: https://chilliant.blogspot.ca/2012/08/srgb-approximations-for-hlsl.html
    let c_linear = clamp(c_linear, 0.0, 1.0);
    // let s1 = c_linear.sqrt();
    // let s2 = s1.sqrt();
    // let s3 = s2.sqrt();
    // let c = 0.662002687 * s1 + 0.684122060 * s2 - 0.323583601 * s3 - 0.0225411470 * c_linear;
    let c = 1.055 * c_linear.powf(0.416666667) - 0.055;
    c.max(0.0)
}

fn u8_to_float(i: u8) -> f32 {
    i as f32 / 255.0
}

fn float_to_u8(f: f32) -> u8 {
    Float::floor(f * 255.0 + 0.5) as u8
}

impl From<Rgb<Rec709>> for sRgb {
    fn from(rgb: Rgb<Rec709>) -> Self {
        sRgb::new(
            linear_to_srgb_approx(rgb.r),
            linear_to_srgb_approx(rgb.g),
            linear_to_srgb_approx(rgb.b))
    }
}

impl From<sRgb> for Rgb<Rec709> {
    fn from(rgb: sRgb) -> Self {
        Rgb::new(
            srgb_to_linear_approx(rgb.r),
            srgb_to_linear_approx(rgb.g),
            srgb_to_linear_approx(rgb.b))
    }
}

#[cfg(test)]
fn rgb_round_trip(rgb: [u8; 3], dist: i32) {
    let srgb = sRgb::from_rgb8(rgb);
    let linear = Rgb::from(srgb);
    let srgb_out = sRgb::from(linear);
    let rgb_out = srgb_out.to_rgb8();
    let diff = ((rgb[0] as i32) - (rgb_out[0] as i32)).abs();
    assert!(diff <= dist, "sRGB conversion too inaccurate: initial = {:?}, converted = {:?}", rgb, rgb_out);
}

#[test]
fn test_round_trip() {
    for i in 0..15 {
        let rgb = [i, i, i];
        rgb_round_trip(rgb, 8);
    }
    for i in 15..40 {
        let rgb = [i, i, i];
        rgb_round_trip(rgb, 5);
    }
    for i in 40..=255 {
        let rgb = [i, i, i];
        rgb_round_trip(rgb, 1);
    }
    rgb_round_trip([45, 27, 122], 3);
}