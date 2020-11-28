pub use super::affine_transform::*;
pub use super::linalg::*;
pub use super::probability::*;
pub use super::tangent::*;

// Re-exports

pub use glam::*;
pub use num_traits::{clamp, Zero};

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.dot(b)
}

pub use std::cmp::{min, max};

// Definitions

pub const EPSILON: f32 = 1e-5;

pub const INV_PI: f32 = ::std::f32::consts::FRAC_1_PI;

pub use ::std::f32::consts::{PI};