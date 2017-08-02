use colour::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub id: u32,
    pub base_colour: Colour,
    pub emittance: Colour,
}

impl Material {
    pub fn new(colour: Colour) -> Self {
        Material { id: 0, base_colour: colour, emittance: Colour::zero() }
    }

    pub fn set_id(&mut self, new_id: u32) {
        self.id = new_id;
    }
}