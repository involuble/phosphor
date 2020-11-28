use crate::math::*;
use crate::colour::*;

pub trait Bsdf {
    fn sample(&self, xi: [f32; 2], basis: &TangentFrame, w_i: Vector3<f32>) -> BsdfSample;
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

// impl<T: ?Sized + Bsdf> Bsdf for &'_ mut T {
//     fn sample(&self, xi: [f32; 2], basis: &TangentFrame, w_i: Vector3<f32>) -> BsdfSample {
//         (**self).sample(xi, basis, w_i)
//     }
// }
// where Self: Sized,

impl<T: ?Sized + Bsdf> Bsdf for Box<T> {
    fn sample(&self, xi: [f32; 2], basis: &TangentFrame, w_i: Vector3<f32>) -> BsdfSample {
        self.as_ref().sample(xi, basis, w_i)
    }
    fn eval(&self, basis: &TangentFrame, w_i: Vector3<f32>, w_o: Vector3<f32>) -> BsdfSample {
        self.as_ref().eval(basis, w_i, w_o)
    }
    
    fn albedo(&self) -> Colour {
        self.as_ref().albedo()
    }
    fn reflectivity(&self) -> f32 {
        self.as_ref().reflectivity()
    }
}
