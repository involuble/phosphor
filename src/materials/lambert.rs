use colour::*;
// use math::*;

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