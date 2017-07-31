use na::*;

use colour::*;

pub struct Light {
    pub colour: Colour,
    pub position: Point3<f32>,
    pub scale: f32,
}

impl Light {
    pub fn new(c: Colour, p: Point3<f32>) {
        Light {
            colour: c,
            position: p,
            scale: 1.0,
        }
    }
}
