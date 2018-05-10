#![allow(dead_code)]

use cgmath::*;

pub fn polar_angles_to_cartesian(theta: f32, phi: f32) -> Vector3<f32> {
    let sin_theta = theta.sin();
    Vector3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), theta.cos())
}

pub fn polar_to_cartesian(sin_theta: f32, cos_theta: f32, phi: f32) -> Vector3<f32> {
    Vector3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
}

pub fn same_hemisphere(a: Vector3<f32>, b: Vector3<f32>) -> bool {
    dot(a, b) > 0.0
}

pub fn same_hemisphere_normal(normal: Vector3<f32>, a: Vector3<f32>, b: Vector3<f32>) -> bool {
    dot(normal, a) * dot(normal, b) > 0.0
}