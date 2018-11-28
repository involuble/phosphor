use super::prelude::{EPSILON};

/// Probability distribution function w.r.t solid angle
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PdfW(pub f32);

impl PdfW {
    // pub fn to_pdf_area(&self, dist_sq: f32, cos_theta: f32) -> PdfA {
    //     if dist_sq < EPSILON {
    //         return PdfA(0.0);
    //     }
    //     PdfA(self.0 * cos_theta / dist_sq)
    // }
}

/// Probability distribution function w.r.t area
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PdfA(pub f32);

impl PdfA {
    pub fn to_pdfw(&self, dist_sq: f32, cos_theta: f32) -> PdfW {
        let abs_cos = cos_theta.abs();
        if abs_cos < EPSILON {
            return PdfW(0.0)
        }
        PdfW(self.0 * dist_sq / abs_cos)
    }
}

pub trait MIS {
    fn combine(&self, other: Self) -> Self;
}

impl MIS for PdfW {
    fn combine(&self, other: Self) -> Self {
        PdfW(power_heuristic(1.0, self.0, 1.0, other.0))
    }
}

fn power_heuristic(n_f: f32, pdf_f: f32, n_g: f32, pdf_g: f32) -> f32 {
    let f = n_f * pdf_f;
    let g = n_g * pdf_g;
    f * f / (f * f + g * g)
}