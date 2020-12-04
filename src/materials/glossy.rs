use super::bsdf::*;
use super::fresnel::*;
use super::ggx::*;
use crate::colour::*;
use crate::math::*;

#[derive(Debug, Clone)]
pub struct Glossy {
    pub specular: SchlickFresnel,
    pub ggx: GGX,
}

impl Bsdf for Glossy {
    fn sample(&self, xi: [f32; 2], basis: &TangentFrame, w_o: Vec3) -> BsdfSample {
        let w_i = self.ggx.sample_vndf(xi, basis, w_o);
        self.eval(basis, w_o, w_i)
    }

    fn eval(&self, basis: &TangentFrame, w_o: Vec3, w_i: Vec3) -> BsdfSample {
        let m = (w_o + w_i).normalize();
        let f = self.specular.fresnel(dot(m, w_i));
        let mut sample = self.ggx.eval(basis, w_o, w_i);
        sample.reflectance *= f;
        sample
    }

    fn albedo(&self) -> Colour {
        todo!()
    }

    fn reflectivity(&self) -> f32 {
        todo!()
    }
}
