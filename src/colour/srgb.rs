#![allow(non_camel_case_types)]
#![allow(dead_code)]

use colour::rgb::*;

use num_traits::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl sRGB {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        sRGB {
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
        sRGB {
            r: u8_to_float(rgb[0]),
            g: u8_to_float(rgb[1]),
            b: u8_to_float(rgb[2]),
        }
    }
}

/// Takes a gamma corrected value in sRGB space and linearises it
fn from_srgb(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        let a: f32 = 0.055;
        Float::powf((c+a)/(1.0+a), 2.4)
    }
}

fn from_srgb_approx(c_srgb: f32) -> f32 {
    c_srgb * (c_srgb * (c_srgb * 0.305306011 + 0.682171111) + 0.012522878)
}

/// Takes a value in linear colour space and gamma corrects it to sRGB space.
/// The input c is a normalised (in [0,1]) float
fn to_srgb(c_linear: f32) -> f32 {
    let c = clamp(c_linear, 0.0, 1.0);
    if c < 0.0031308 {
        12.92 * c
    } else {
        let a: f32 = 0.055;
        (1.0+a) * Float::powf(c, 1.0/2.4) - a
    }
}

fn to_srgb_approx(c_linear: f32) -> f32 {
    let c_linear = clamp(c_linear, 0.0, 1.0);
    let s1 = c_linear.sqrt();
    let s2 = s1.sqrt();
    let s3 = s2.sqrt();
    0.662002687 * s1 + 0.684122060 * s2 - 0.323583601 * s3 - 0.0225411470 * c_linear
}

fn u8_to_float(i: u8) -> f32 {
    i as f32 / 255.0
}

fn float_to_u8(f: f32) -> u8 {
    Float::floor(f * 255.0 + 0.5) as u8
}

impl From<RGB<Rec709>> for sRGB {
    fn from(rgb: RGB<Rec709>) -> Self {
        sRGB {
            r: to_srgb_approx(rgb.r),
            g: to_srgb_approx(rgb.g),
            b: to_srgb_approx(rgb.b),
        }
    }
}

impl From<sRGB> for RGB<Rec709> {
    fn from(rgb: sRGB) -> Self {
        RGB::new(from_srgb_approx(rgb.r), from_srgb_approx(rgb.g), from_srgb_approx(rgb.b))
    }
}

#[test]
fn test_round_trip() {
    let rgb = [45, 27, 122];
    let srgb = sRGB::from_rgb8(rgb);
    let rgb_out = srgb.to_rgb8();
    assert_eq!(rgb, rgb_out);
}