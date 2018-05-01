use colour::*;

pub use materials::*;

#[derive(Debug, Clone)]
pub enum Material {
    None,
    Lambert(Lambert),
}

impl Bsdf for Material {
    fn albedo(&self) -> Colour {
        match *self {
            Material::Lambert(ref l) => l.albedo,
            Material::None => Colour::new(0.0, 0.0, 0.0),
        }
    }
}
