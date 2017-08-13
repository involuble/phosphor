#![allow(dead_code)]

use na::*;
use std::f32;
use std::f32::consts::{PI, FRAC_1_PI};
use std::default::{Default};
use rand;

pub struct CosineHemisphereSampler;

impl CosineHemisphereSampler {
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
}

pub struct UniformHemisphereSampler;

impl UniformHemisphereSampler {
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

pub struct UniformConeSampler;

impl UniformConeSampler {
    pub fn sample<R: rand::Rng>(rng: &mut R, cos_theta_max: f32) -> (Vector3<f32>, f32) {
        let u1 = rng.next_f32();
        let u2 = rng.next_f32();

        let cos_theta = (1.0 - u1) + u1*cos_theta_max;
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let phi = 2.0 * PI * u2;

        (Vector3::new(phi.cos() * sin_theta, phi.sin() * sin_theta, cos_theta), 1.0 / (2.0 * PI * (1.0 - cos_theta_max)))
    }
}

// https://en.wikipedia.org/wiki/Low-discrepancy_sequence#Additive_recurrence
// https://blog.demofox.org/2017/05/29/when-random-numbers-are-too-random-low-discrepancy-sequences/
pub struct AdditiveRecurrence {
    pub f: f32,
}

impl Default for AdditiveRecurrence {
    fn default() -> Self { AdditiveRecurrence { f: 0.5 }}
}

impl AdditiveRecurrence {
    pub fn seed(seed: f32) -> Self {
        AdditiveRecurrence { f: seed }
    }

    pub fn generate(&mut self) -> f32 {
        const BASE: f32 = 1.61803398875;
        let r = self.f;
        self.f = (self.f + BASE) % 1.0;
        r
    }
}

// TODO:
// Reference: http://web.maths.unsw.edu.au/~fkuo/sobol/
pub struct SobolSequence;