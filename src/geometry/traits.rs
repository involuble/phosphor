use crate::math::*;
use crate::colour::*;

pub trait SampleableEmitter: Send + Sync {
    fn eval_emission_at(&self, initial: Vec3, p: Vec3) -> LightSample;
    fn sample(&self, xi: [f32; 2], initial: Vec3) -> LightSample;

    fn surface_area(&self) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct LightSample {
    /// Direction to the sampled point on the object
    pub dir: Vec3,
    /// A conservative estimate of the distance to the sampled object
    pub distance: f32,
    pub radiance: Colour,
    pub pdf: PdfW,
}