use math::*;
use colour::*;
use sampling::*;

pub trait SampleableEmitter: Send + Sync {
    fn eval_emission_at(&self, initial: Point3<f32>, p: Point3<f32>) -> LightSample;
    fn sample(&self, rng: &mut SampleRng, initial: Point3<f32>) -> LightSample;

    fn surface_area(&self) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct LightSample {
    /// Direction to the sampled point on the object
    pub dir: Vector3<f32>,
    /// A conservative estimate of the distance to the sampled object
    pub distance: f32,
    pub radiance: Colour,
    pub pdf: PdfW,
}