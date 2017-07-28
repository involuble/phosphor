#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub g: f32,
    pub b: f32,
    pub r: f32,
}

impl Colour {
    pub fn from_luma(y: f32) -> Self {
        Colour { r: y, g: y, b: y }
    }

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Colour { r: r, g: g, b: b }
    }

    pub fn black() -> Self {
        Colour { r: 0.0, g: 0.0, b: 0.0 }
    }
}