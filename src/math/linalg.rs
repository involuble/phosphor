use glam::*;
use super::prelude::dot;

pub fn polar_angles_to_cartesian(theta: f32, phi: f32) -> Vec3 {
    let sin_theta = theta.sin();
    Vec3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), theta.cos())
}

pub fn polar_to_cartesian(sin_theta: f32, cos_theta: f32, phi: f32) -> Vec3 {
    Vec3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
}

pub fn same_hemisphere(a: Vec3, b: Vec3) -> bool {
    dot(a, b) > 0.0
}

pub fn same_hemisphere_normal(normal: Vec3, a: Vec3, b: Vec3) -> bool {
    dot(normal, a) * dot(normal, b) > 0.0
}