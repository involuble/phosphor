use embree;

// use colour::*;

use crate::materials::*;

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