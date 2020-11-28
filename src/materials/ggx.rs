#![allow(dead_code)]

use super::bsdf::*;
use crate::math::*;
use crate::colour::*;

#[derive(Debug, Clone, Copy)]
pub struct GGX {
    pub alpha: f32,
}

impl GGX {
    fn lambda(&self, basis: &TangentFrame, v: Vec3) -> f32 {
        // Section 5.3 Eq. (72) in [Heitz2014Microfacet]
        let vdotn = dot(basis.normal, v);

        let cos_theta_sq = vdotn * vdotn;
        let tan_theta_sq = 1.0 / cos_theta_sq - 1.0;
        let tan_theta = tan_theta_sq.sqrt();

        if tan_theta.is_infinite() {
            return 0.0;
        }

        let a = 1.0 / (self.alpha * tan_theta);

        (-1.0 + (1.0 + 1.0/(a*a)).sqrt()) / 2.0
    }

    #[allow(non_snake_case)]
    fn G(&self, basis: &TangentFrame, w_i: Vec3, w_o: Vec3, m:  Vec3) -> f32 {
        // This is the height correlated masking and shadowing function for GGX
        // See Eq. (99)
        // Ref: Understanding the Masking-Shadowing Function in Microfacet-Based BRDFs
        //  [Heitz2014Microfacet]
        // χ+(a) is the Heaviside function: 1 if a > 0, 0 if a <= 0
        
        let odotm = dot(w_o, m);
        let idotm = dot(w_i, m);

        // χ+(ωo·ωm) * χ+(ωi·ωm)
        if odotm <= 0.0 || idotm <= 0.0 {
            return 0.0
        }

        1.0 / (1.0 + self.lambda(basis, w_i) + self.lambda(basis, w_o))
    }

    fn ndf(&self, basis: &TangentFrame, m: Vec3) -> f32 {
        let alpha_sq = self.alpha * self.alpha;
        let cos_h = dot(basis.normal, m);

        // χ+(m·n)
        if cos_h <= 0.0 {
            return 0.0;
        }
        let denom = 1.0 + (alpha_sq - 1.0) * cos_h * cos_h;
        alpha_sq / (PI * denom * denom)
    }

    fn sample(&self, _xi: [f32; 2], _basis: &TangentFrame, w_i: Vec3) -> BsdfSample {
        let _w_i = Vec3::new(w_i.x * self.alpha, w_i.y * self.alpha, w_i.z);
        unimplemented!()
    }

    fn eval(&self, basis: &TangentFrame, w_i: Vec3, w_o: Vec3) -> BsdfSample {
        let m = (w_o + w_i).normalize();
        
        let idotn = dot(w_i, basis.normal);
        let odotn = dot(w_o, basis.normal);

        let odotm = dot(w_o, m);

        let ndotm = dot(basis.normal, m);

        let d = self.ndf(basis, m);
        let g = self.G(basis, w_i, w_o, m);
        // let f = self.spec.fresnel(odotm);

        let c = d * g / (4.0 * idotn * odotn);

        let pdf = ndotm * d / (4.0 * odotm);

        BsdfSample {
            reflectance: Colour::new(c, c, c),
            w_o: w_o,
            pdf: PdfW(pdf),
        }
    }
}
