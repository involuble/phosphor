use rand::{IsaacRng};

use math::*;
use colour::*;

pub trait SampleableEmitter: Send + Sync {
    fn eval_emission_at(&self, initial: Point3<f32>, p: Point3<f32>) -> LightSample;
    fn sample(&self, rng: &mut IsaacRng, initial: Point3<f32>) -> LightSample;

    fn surface_area(&self) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct LightSample {
    pub dir: Vector3<f32>,
    pub distance: f32,
    pub radiance: Colour,
    pub pdf: PdfW,
}