use embree;

// use colour::*;

pub use materials::*;

#[derive(Debug, Clone)]
pub enum MaterialType {
    Diffuse(Lambert),
}

impl Material for MaterialType {
    fn compute_bsdf(&self, _hit: &embree::Hit) -> Box<Bsdf> {
        match *self {
            MaterialType::Diffuse(ref l) => Box::new(l.clone()),
        }
    }
}