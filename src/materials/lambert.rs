use rand::{Rng, IsaacRng};

use super::bsdf::*;
use colour::*;
use math::*;

#[derive(Debug, Clone)]
pub struct Lambert {
    pub albedo: Colour,
}

impl Lambert {
    pub fn new(albedo: Colour) -> Self {
        Lambert {
            albedo: albedo,
        }
    }
}

fn sample_cos_hempisphere(rng: &mut IsaacRng) -> (Vector3<f32>, f32) {
    let u1 = rng.next_f32();
    let u2 = rng.next_f32();

    let r = u1.sqrt();
    let theta = 2.0 * PI * u2;

    let c_t = theta.cos();
    let s_t = theta.sin();

    let z = (1.0 - u1).sqrt();

    (Vector3::new(r * c_t, r * s_t, z), z)
}

impl Bsdf for Lambert {
    fn sample(&self, rng: &mut IsaacRng, basis: &TangentFrame, _w_i: Vector3<f32>) -> BsdfSample {
        let (w_o_local, z) = sample_cos_hempisphere(rng);
        BsdfSample {
            reflectance: self.albedo * INV_PI,
            w_o: basis.transform(w_o_local).normalize(),
            pdf: PdfW(z * INV_PI),
        }
    }

    fn eval(&self, basis: &TangentFrame, _w_i: Vector3<f32>, w_o: Vector3<f32>) -> BsdfSample {
        BsdfSample {
            reflectance: self.albedo * INV_PI,
            w_o: w_o,
            pdf: PdfW(dot(basis.normal, w_o) * INV_PI),
        }
    }

    fn albedo(&self) -> Colour {
        self.albedo
    }
    
    fn reflectivity(&self) -> f32 {
        0.0
    }
}