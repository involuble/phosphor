use embree;

use crate::math::*;
use crate::colour::*;
use crate::sampling::*;

pub trait Bsdf {
    fn sample(&self, rng: &mut SampleRng, basis: &TangentFrame, w_i: Vector3<f32>) -> BsdfSample;
    fn eval(&self, basis: &TangentFrame, w_i: Vector3<f32>, w_o: Vector3<f32>) -> BsdfSample;
    
    fn albedo(&self) -> Colour;
    fn reflectivity(&self) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct BsdfSample {
    pub reflectance: Colour,
    pub w_o: Vector3<f32>,
    pub pdf: PdfW,
}

pub trait Material {
    fn compute_bsdf(&self, hit: &embree::Hit) -> Box<Bsdf>;
}
