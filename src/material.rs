use colour::*;
use materials::*;

#[derive(Debug, Clone)]
pub enum Material {
    Emitter,
    Lambert(Lambert),
}

impl Bsdf for Material {
    fn albedo(&self) -> Colour {
        match *self {
            Material::Emitter => Colour::new(1.0, 1.0, 1.0),
            Material::Lambert(ref l) => l.albedo
        }
    }
}
