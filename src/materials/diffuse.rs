use super::bsdf::*;
use crate::colour::*;
use crate::math::*;

#[derive(Debug, Clone)]
pub struct Lambert {
    pub albedo: Colour,
}

impl Lambert {
    pub fn new(albedo: Colour) -> Self {
        Lambert {
            albedo,
        }
    }
}

fn sample_cos_hempisphere(xi: [f32; 2]) -> (Vec3, f32) {
    let u1 = xi[0];
    let u2 = xi[1];

    let r: f32 = u1.sqrt();
    let theta = 2.0 * PI * u2;

    let c_t = theta.cos();
    let s_t = theta.sin();

    let z = (1.0 - u1).sqrt();

    (Vec3::new(r * c_t, r * s_t, z), z)
}

impl Bsdf for Lambert {
    fn sample(&self, xi: [f32; 2], basis: &TangentFrame, _w_o: Vec3) -> BsdfSample {
        let (w_i_local, z) = sample_cos_hempisphere(xi);
        BsdfSample {
            reflectance: self.albedo * INV_PI,
            w_i: basis.transform(w_i_local).normalize(),
            pdf: PdfW(z * INV_PI),
        }
    }

    fn eval(&self, basis: &TangentFrame, _w_o: Vec3, w_i: Vec3) -> BsdfSample {
        BsdfSample {
            reflectance: self.albedo * INV_PI,
            w_i: w_i,
            pdf: PdfW(dot(basis.normal, w_i) * INV_PI),
        }
    }

    fn albedo(&self) -> Colour {
        self.albedo
    }
    
    fn reflectivity(&self) -> f32 {
        0.0
    }
}