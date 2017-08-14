use colour::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub base_colour: Colour,
    pub emittance: Colour,
}

impl Material {
    pub fn new(colour: Colour) -> Self {
        Material { base_colour: colour, emittance: Colour::zero() }
    }

    pub fn new_emitter(e: Colour) -> Self {
        Material { base_colour: Colour::black(), emittance: e }
    }
}