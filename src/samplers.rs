use na::*;
use std::f32;
use std::f32::consts::PI;
use std::default::{Default};
use rand;

// Creates an orthonormal basis given a normal vector.
//   The vectors are returned in a tuple as tangent and bitangent
// Reference: http://jcgt.org/published/0006/01/01/paper.pdf
//   (same paper) http://graphics.pixar.com/library/OrthonormalB/paper.pdf 
pub fn orthonormal_basis(n: Vector3<f32>) -> (Vector3<f32>, Vector3<f32>) {
    let sign = n.z.signum();
    let a = -1.0 / (sign + n.z);
    let b = n.x * n.y * a;
    let b1 = Vector3::new(1.0 + sign*n.x*n.x*a, sign*b, -sign*n.x);
    let b2 = Vector3::new(b, sign + n.y*n.y*a, -n.y);
    (b1, b2)
}

pub struct CosineHemisphereSampler;

impl CosineHemisphereSampler {
    // Reference: http://www.rorydriscoll.com/2009/01/07/better-sampling/
    pub fn sample<R: rand::Rng>(&self, rng: &mut R) -> Vector3<f32> {
        let u1 = rng.next_f32();
        let u2 = rng.next_f32();

        let r = u1.sqrt();
        let theta = 2.0 * PI * u2;

        let x = r * theta.cos();
        let y = r * theta.sin();

        Vector3::new(x, y, (1.0 - u1).sqrt())
    }
}

pub struct UniformHemisphereSampler;

impl UniformHemisphereSampler {
    // Reference: http://www.rorydriscoll.com/2009/01/07/better-sampling/
    pub fn sample<R: rand::Rng>(&self, rng: &mut R) -> Vector3<f32> {
        let u1 = rng.next_f32();
        let u2 = rng.next_f32();

        let r = (1.0 - u1*u1).sqrt();
        let phi = 2.0 * PI * u2;

        let x = r * phi.cos();
        let y = r * phi.sin();

        Vector3::new(x, y, u1)
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