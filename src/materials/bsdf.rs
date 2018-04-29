use math::*;
use colour::*;

pub trait Bsdf {
    // TODO
    // fn sample(&self) -> BsdfSample;
    
    fn albedo(&self) -> Colour;
}

#[derive(Debug, Clone, Copy)]
pub struct BsdfSample {
    reflectance: Colour,
    w_o: Vector3<f32>,
    pdf: PdfW,
}