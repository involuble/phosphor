use na::*;
use std::f32::consts::PI;

use rand;

pub struct CosineHemisphereSampler;

impl CosineHemisphereSampler {
    // See http://www.rorydriscoll.com/2009/01/07/better-sampling/
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