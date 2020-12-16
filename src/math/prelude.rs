pub use super::affine_transform::*;
pub use super::linalg::*;
pub use super::probability::*;
pub use super::tangent::*;

// Re-exports

pub use glam::*;
pub use num_traits::{clamp, Zero};

pub type Point3 = Vec3;

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.dot(b)
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    -v + 2.0 * dot(v, n) * n
}

pub fn refract(v: Vec3, n: Vec3, eta: f32) -> Vec3 {
    let cos_theta_v = dot(v, n);
    let k = 1.0 - eta * eta * (1.0 - cos_theta_v * cos_theta_v);
    if k < 0.0 {
        Vec3::zero()
    } else {
        v * eta - (eta * cos_theta_v + k.sqrt()) * n
    }
}

pub use std::cmp::{min, max};

// Definitions

pub const EPSILON: f32 = 1e-5;

pub const INV_PI: f32 = ::std::f32::consts::FRAC_1_PI;

pub use std::f32::consts::PI;