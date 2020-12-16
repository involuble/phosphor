#![allow(non_snake_case)]

use crate::math::*;

#[derive(Debug, Clone, Copy)]
pub struct GGX {
    alpha: f32,
}

impl GGX {
    pub fn new(roughness: f32) -> Self {
        GGX {
            alpha: (roughness * roughness).max(0.001),
        }
    }
}

impl GGX {
    fn lambda(&self, basis: &TangentFrame, v: Vec3) -> f32 {
        // Section 5.3 Eq. (72) in [Heitz2014Microfacet]
        let alpha_sq = self.alpha * self.alpha;
        let vdotn = dot(basis.normal, v);

        let cos_theta_sq = vdotn * vdotn;
        let tan_theta_sq = (1.0 / cos_theta_sq) - 1.0;

        // if tan_theta_sq <= 0.0 {
        //     return 0.0;
        // }
        debug_assert!(!tan_theta_sq.is_infinite());

        let inv_a_sq = alpha_sq*tan_theta_sq;

        (-1.0 + (1.0 + inv_a_sq).sqrt()) / 2.0
    }

    fn G1(&self, basis: &TangentFrame, w: Vec3) -> f32 {
        1.0 / (1.0 + self.lambda(basis, w))
    }

    fn G2(&self, basis: &TangentFrame, w_o: Vec3, w_i: Vec3, m: Vec3) -> f32 {
        // This is the height correlated masking and shadowing function for GGX
        // See Section 6, Eq. (99)
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

    pub fn sample_vndf(&self, xi: [f32; 2], basis: &TangentFrame, w_o: Vec3) -> Vec3 {
        // See [Heitz18]: Sampling the GGX Distribution of Visible Normals

        let Ve = basis.inv_transform(w_o);

        // transforming the view direction to the hemisphere configuration
        let Vh = Vec3::new(Ve.x * self.alpha, Ve.y * self.alpha, Ve.z);

        // orthonormal basis
        let len_sq = Vh.x * Vh.x + Vh.y * Vh.y;
        let T1 = if len_sq > 0.0 { Vec3::new(-Vh.y, Vh.x, 0.0) / len_sq.sqrt() } else { Vec3::unit_x() };
        let T2 = Vh.cross(T1);

        // parameterization of the projected area
        let r = xi[0].sqrt();
        let phi = 2.0 * PI * xi[1];
        let t1 = r * phi.cos();
        let t2 = r * phi.sin();
        let s = 0.5 * (1.0 + Vh.z);
        let t2 = (1.0 - s) * (1.0 - t1*t1).sqrt() + s*t2;

        // reprojection onto hemisphere
        let Nh: Vec3 = t1*T1 + t2*T2 + (1.0 - t1*t1 - t2*t2).max(0.0).sqrt()*Vh;
        let Ne = Vec3::new(Nh.x * self.alpha, Nh.y * self.alpha, Nh.z.max(0.0)).normalize();

        basis.transform(Ne)
    }

    pub fn eval(&self, basis: &TangentFrame, w_o: Vec3, w_i: Vec3) -> (f32, f32) {
        let m = (w_o + w_i).normalize();
        
        let idotn = dot(w_i, basis.normal);
        let odotn = dot(w_o, basis.normal);

        let odotm = dot(w_o, m);

        let d = self.ndf(basis, m);
        let g = self.G2(basis, w_o, w_i, m);

        let denom = 4.0 * idotn * odotn;

        let c = d * g / denom;

        // [Heitz18] says:
        //   Output Ne: normal sampled with PDF D_Ve(Ne) = G1(Ve) * max(0, dot(Ve, Ne)) * D(Ne) / Ve.z
        // Using the separable form of G2 from [Heitz14] Eq. 98, G1(ωo) = χ+(ωo · ωm) / 1+Λ(ωo)
        // So PDF = G1(w_o) * max(0, odotm) * D(w_m) / odotn / (4 * odotm)
        // let pdf = ndotm * d / (4.0 * odotm);
        let pdf = self.G1(basis, w_o) * d * odotm.max(0.0) / denom;

        (c, pdf)
    }
}
