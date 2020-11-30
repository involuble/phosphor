use crate::math::*;
use crate::colour::*;

pub trait Bsdf {
    fn sample(&self, xi: [f32; 2], basis: &TangentFrame, w_o: Vec3) -> BsdfSample;
    fn eval(&self, basis: &TangentFrame, w_o: Vec3, w_i: Vec3) -> BsdfSample;
    
    fn albedo(&self) -> Colour;
    fn reflectivity(&self) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct BsdfSample {
    pub reflectance: Colour,
    pub w_i: Vec3,
    pub pdf: PdfW,
}

// impl<T: ?Sized + Bsdf> Bsdf for &'_ mut T {
//     fn sample(&self, xi: [f32; 2], basis: &TangentFrame, w_i: Vec3) -> BsdfSample {
//         (**self).sample(xi, basis, w_i)
//     }
// }
// where Self: Sized,

impl<T: ?Sized + Bsdf> Bsdf for Box<T> {
    fn sample(&self, xi: [f32; 2], basis: &TangentFrame, w_o: Vec3) -> BsdfSample {
        self.as_ref().sample(xi, basis, w_o)
    }
    fn eval(&self, basis: &TangentFrame, w_o: Vec3, w_i: Vec3) -> BsdfSample {
        self.as_ref().eval(basis, w_o, w_i)
    }
    
    fn albedo(&self) -> Colour {
        self.as_ref().albedo()
    }
    fn reflectivity(&self) -> f32 {
        self.as_ref().reflectivity()
    }
}
