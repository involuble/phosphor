use na::*;

use colour::*;

pub struct PointLight {
    pub colour: Colour,
    pub position: Point3<f32>,
    pub scale: f32,
}

impl PointLight {
    pub fn new(c: Colour, p: Point3<f32>) -> Self {
        PointLight {
            colour: c,
            position: p,
            scale: 1.0,
        }
    }
}
