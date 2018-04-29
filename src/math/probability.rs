use std::f32::EPSILON;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PdfW(pub f32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PdfA(f32);

impl PdfA {
    pub fn to_pdf_solid_angle(&self, dist_sq: f32, cos_theta: f32, surface_area: f32) -> PdfW {
        debug_assert!(surface_area > EPSILON);
        let abs_cos = cos_theta.abs();
        if abs_cos <= 0.0 {
            return PdfW(0.0)
        }
        // TODO: I don't think this is quite right
        PdfW(self.0 * dist_sq / (abs_cos * surface_area))
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