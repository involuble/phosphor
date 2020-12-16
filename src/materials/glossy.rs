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
        let w_m = self.ggx.sample_vndf(xi, basis, w_o);
        let w_i = reflect(w_o, w_m);
        self.eval(basis, w_o, w_i)
    }

    fn eval(&self, basis: &TangentFrame, w_o: Vec3, w_i: Vec3) -> BsdfSample {
        let m = (w_o + w_i).normalize();
        let f = self.specular.fresnel(dot(m, w_i));
        let (c, pdf) = self.ggx.eval(basis, w_o, w_i);
        BsdfSample {
            reflectance: c * f,
            w_i: w_i,
            pdf: PdfW(pdf),
        }
    }

    fn albedo(&self) -> Colour {
        todo!()
    }

    fn reflectivity(&self) -> f32 {
        todo!()
    }
}
