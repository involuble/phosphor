use na::*;
use std::f32::consts::PI;
use std::f32::{sqrt, abs};
use rand;

use brdf::*;
use linalg::*;
use material::*;
use materialparams::*;
use microfacet::*;
use sample::*;

// Reference:
// http://graphicrants.blogspot.ca/2013/08/specular-brdf-reference.html
// [Walter07] Microfacet Models for Refraction through Rough Surfaces. Bruce Walter et al
//    https://www.cs.cornell.edu/~srm/publications/EGSR07-btdf.pdf
// [Burley12] Physically-based Shading at Disney
//    http://blog.selfshadow.com/publications/s2012-shading-course/burley/s2012_pbs_disney_brdf_notes_v3.pdf
// [Heitz2014Microfacet] Understanding the Masking-Shadowing Function in Microfacet-Based BRDFs
//    https://hal.inria.fr/file/index/docid/942452/filename/RR-8468.pdf
//    http://jcgt.org/published/0003/02/03/paper.pdf

pub struct GGXNormalDistribution;

impl NormalDistFn for GGXNormalDistribution {
    pub fn ndf(n: Vector3<f32>, alpha: f32, half: Vector3<f32>) -> f32 {
        let alpha_sq = alpha * alpha;
        let cos_h = dot(&n, &half);

        // χ+(m·n)
        if cos_h <= 0.0 {
            return 0.0;
        }
        let denom = 1.0 + (alpha_sq - 1.0) * cos_h * cos_h;
        alpha_sq / (PI * denom * denom)
    }
}

impl GGXNormalDistribution {
    pub fn sample_micronormal<R: rand::Rng>(rng: &mut R, alpha: f32) -> (f32, f32) {
        let u1 = rng.next_f32();
        let u2 = rng.next_f32();

        let theta = (alpha * (u1 / (1.0 - u1)).sqrt()).atan();
        let phi = 2.0 * PI * u2;

        (theta, phi)
    }
}

pub struct SmithGGXShadowMask;

// Note: χ+ is the the Heaviside function
//    χ+(a) = 1 if a > 0 and 0 if a <= 0
impl SmithGGXShadowMask {
    fn g1(n: Vector3<f32>, alpha: f32, vec: Vector3<f32>, half: Vector3<f32>) -> f32 {
        let vdoth = dot(&vec, &half);
        let vdotn = dot(&vec, &n);

        // χ+(v·m/v·n)
        if vdoth * vdotn <= 0.0 {
            return 0.0
        }

        let alpha_sq = alpha * alpha;
        let cos_theta_sq = vdotn * vdotn;
        let tan_theta_sq = 1.0 / cos_theta_sq - 1.0;
        let denom = 1.0 + (1.0 + alpha_sq * tan_theta_sq).sqrt();
        2.0 / denom
    }
}

impl ShadowingFn for SmithGGXShadowMask {
    pub fn shadowing(n: Vector3<f32>, alpha: f32, view: Vector3<f32>, light: Vector3<f32>, half: Vector3<f32>) -> f32 {
        Self::g1(n, alpha, light, half) * Self::g1(n, alpha, view, half)
    }
}

pub struct HeightCorrelatedSmith;

impl HeightCorrelatedSmith {
    // Section 3 in [Heitz2014Microfacet]
    fn lambda(n: Vector3<f32>, alpha: f32, vec: Vector3<f32>) -> f32 {
        let vdotn = dot(&n, &vec);
        let cos_theta_sq = vdotn * vdotn;
        let tan_theta_sq = 1.0 / cos_theta_sq - 1.0;
        let tan_theta = tan_theta_sq.sqrt();
        let a = 1.0 / (alpha * tan_theta)
        (-1.0 + (1.0 + 1.0/(a*a)).sqrt()) / 2.0
    }
}

// Eq. (21) in [Heitz2014Microfacet]
impl ShadowingFn for HeightCorrelatedSmith {
    pub fn shadowing(n: Vector3<f32>, alpha: f32, view: Vector3<f32>, light: Vector3<f32>, half: Vector3<f32>) -> f32 {
        let ldotn = dot(&light, &n);
        let vdotn = dot(&view, &n);

        // χ+(ωo·ωn) * χ+(ωi·ωn)
        if ldotn <= 0.0 || vdotn <= 0.0 {
            return 0.0
        }

        1.0 / (1.0 + lambda(n, alpha, view) + lambda(n, alpha, light))
    }
}

pub struct GGX;

impl GGX {
    pub fn sample_m_naive(&self, params: &BRDFParams, rng: &mut R, view: Vector3<f32>) -> Vector3<f32> {
        let (theta, phi) = GGXNormalDistribution::sample_micronormal(rng, params.mat.roughness);
        let m_shading = spherical_direction(theta, phi);

        let m = m_shading.x * params.tangent + m_shading.y * params.bitang + m_shading.z * params.n;
        let m = m.normalize();
        2.0 * abs(dot(&view, &m)) * m - view
    }

    pub fn pdf(&self, params: &BRDFParams, view: Vector3<f32>, w_i: Vector3<f32>) -> f32 {
        let m = (view + w_i).normalize();
        let vdotm = abs(dot(&view, &m));
        let ndotm = abs(dot(&params.n, &m));
        ndotm * ndf(params.n, params.mat.roughness, m) / (4.0 * vdotm)
    }

    pub fn sample(&self, params: &BRDFParams, rng: &mut R, view: Vector3<f32>) -> BRDFSample {
        let w_i = self.sample_m_naive(params, rng, view);
        self.eval(params, rng, view, w_i)
    }

    pub fn eval(&self, params: &BRDFParams, view: Vector3<f32>, light: Vector3<f32>) -> BRDFSample {
        let m = (view + w_i).normalize();
        
        let idotn = dot(&light, &n);
        let odotn = dot(&view, &n);

        let g = HeightCorrelatedSmith::shadowing(params.n, params.mat.rougness, view, light, m);
        let d = GGXNormalDistribution::ndf(params.n, params.mat.roughness, m);
        
        let c = g * d / (4.0 * idotn * odotn);
        let pdf = self.pdf(params, view, light);

        BRDFSample { w_i: light, refl: Colour::from_luma(c), pdf: pdf }
    }
}