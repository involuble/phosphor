use crate::colour::*;

// See https://seblagarde.wordpress.com/2013/04/29/memo-on-fresnel-equations/
//  for a reference on the Fresnel equations

/// Complex index of refraction
/// The 3 numbers correspond to values at 650nm, 550nm and 450nm, respectively (red, green and blue)
pub struct Ior {
    /// Refractive index
    pub n: [f32; 3],
    /// Extinction coefficient
    pub k: [f32; 3],
}

#[derive(Debug, Clone, Copy)]
pub struct SchlickFresnel {
    // TODO: Not sure if this makes sense as a colour
    r0: Colour,
}

impl SchlickFresnel {
    pub fn new(ior: Ior) -> Self {
        fn r0(n: f32, k: f32) -> f32 {
            let k_sq = k * k;
            let n_minus = (n - 1.0) * (n - 1.0);
            let n_plus = (n + 1.0) * (n + 1.0);
            (n_minus + k_sq) / (n_plus + k_sq)
        }
        SchlickFresnel {
            r0: Colour::new(r0(ior.n[0], ior.k[0]), r0(ior.n[1], ior.k[1]), r0(ior.n[2], ior.k[2])),
        }
    }
    
    pub fn fresnel(&self, cos_t: f32) -> Colour {
        self.r0 + (Colour::one() - self.r0) * (1.0 - cos_t).powf(5.0)
    }
}

pub struct DielectricFresnel {
    pub n: f32,
}

impl DielectricFresnel {
    pub fn new(n: f32) -> Self {
        DielectricFresnel {
            n: n,
        }
    }

    pub fn fresnel(&self, cos_t: f32) -> f32 {
        let sin_theta_sq = 1.0 - cos_t * cos_t;
        let s = (1.0 - sin_theta_sq / (self.n * self.n)).sqrt();
        // r⊥
        let rs = (cos_t - self.n * s) / (cos_t + self.n * s);
        // r||
        let rp = (s - self.n * cos_t) / (s + self.n * cos_t);
        0.5 * (rs * rs + rp * rp)
    }
}