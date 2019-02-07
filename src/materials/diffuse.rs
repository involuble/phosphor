use super::bsdf::*;
use crate::colour::*;
use crate::math::*;
use crate::sampling::*;

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

fn sample_cos_hempisphere<R: Rng>(rng: &mut R) -> (Vector3<f32>, f32) {
    let u1: f32 = rng.gen();
    let u2: f32 = rng.gen();

    let r: f32 = u1.sqrt();
    let theta = 2.0 * PI * u2;

    let c_t = theta.cos();
    let s_t = theta.sin();

    let z = (1.0 - u1).sqrt();

    (Vector3::new(r * c_t, r * s_t, z), z)
}

impl Bsdf for Lambert {
    fn sample(&self, rng: &mut SampleRng, basis: &TangentFrame, _w_i: Vector3<f32>) -> BsdfSample {
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