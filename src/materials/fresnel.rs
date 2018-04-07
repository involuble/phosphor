use std::f32::{powi};
use cgmath::*;

use colour::*;

pub trait SchlickFresnel;

impl SchlickFresnel {
    pub fn dielectric(f0: f32, light: Vector3<f32>, half: Vector3<f32>) -> f32 {
        let cos_t = dot(&light, &half);
        f0 + (1.0 - f0)*(1.0 - powi(cos_t,5))
    }
}