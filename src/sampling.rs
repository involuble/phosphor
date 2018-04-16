#![allow(dead_code)]

use cgmath::*;
use std::f32;
use std::f32::consts::{PI, FRAC_1_PI};
use rand;

use linalg::*;

pub struct CosineHemisphereDistribution;

impl CosineHemisphereDistribution {
    // Reference: http://www.rorydriscoll.com/2009/01/07/better-sampling/
    pub fn sample<R: rand::Rng>(rng: &mut R) -> (Vector3<f32>, f32) {
        let u1 = rng.next_f32();
        let u2 = rng.next_f32();

        let r = u1.sqrt();
        let theta = 2.0 * PI * u2;

        let c_t = theta.cos();
        let s_t = theta.sin();

        let z = (1.0 - u1).sqrt();
        (Vector3::new(r * c_t, r * s_t, z), z * FRAC_1_PI)
    }

    // Reference: http://amietia.com/lambertnotangent.html
    pub fn sample_world_space<R: rand::Rng>(rng: &mut R, normal: Vector3<f32>) -> Vector3<f32> {
        let u1 = rng.next_f32();
        let u2 = rng.next_f32();

        let u = 2.0 * u1 - 1.;
        let theta = 2.0 * PI * u2;

        let d = (1.0 - u*u).sqrt();

        let sphere_point = Vector3::new(d * theta.cos(), d * theta.sin(), u);
        (normal + sphere_point).normalize()
    }
}

pub struct UniformHemisphereDistribution;

impl UniformHemisphereDistribution {
    pub fn sample<R: rand::Rng>(rng: &mut R) -> (Vector3<f32>, f32) {
        let u1 = rng.next_f32();
        let u2 = rng.next_f32();

        let r = (1.0 - u1*u1).sqrt();
        let phi = 2.0 * PI * u2;

        let x = r * phi.cos();
        let y = r * phi.sin();

        (Vector3::new(x, y, u1), 0.5 * FRAC_1_PI)
    }
}

pub struct UniformConeDistribution;

impl UniformConeDistribution {
    pub fn sample<R: rand::Rng>(rng: &mut R, cos_theta_max: f32) -> (Vector3<f32>, f32) {
        let u1 = rng.next_f32();
        let u2 = rng.next_f32();

        let cos_theta = (1.0 - u1) + u1*cos_theta_max;
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let phi = 2.0 * PI * u2;

        (spherical_dir_from_sincos(sin_theta, cos_theta, phi), 1.0 / (2.0 * PI * (1.0 - cos_theta_max)))
    }
}
