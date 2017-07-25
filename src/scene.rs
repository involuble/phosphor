use primitive::*;

pub struct Scene {
    prims: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            prims: Vec::new(),
        }
    }
}