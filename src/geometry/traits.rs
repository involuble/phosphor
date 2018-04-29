use math::*;
use colour::*;

pub trait AreaLight {
    fn eval_emission_at(&self, initial: Point3<f32>, p: Point3<f32>) -> LightSample;
    fn sample(&self, initial: Point3<f32>) -> LightSample;

    // fn surface_area(&self) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct LightSample {
    pub direction: Vector3<f32>,
    pub radiance: Colour,
    pub pdf: PdfW,
}