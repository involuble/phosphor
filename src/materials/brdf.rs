use cgmath::*;

use colour::*;

pub struct BRDFSample {
    // Incoming direction
    pub w_i: Vector3<f32>,
    // Reflectance
    pub refl: Colour,
    pub pdf: f32,
}