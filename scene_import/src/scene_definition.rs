
// pub type Colour = [f32; 3];

#[derive(Debug, Clone)]
pub struct Scene {
    pub media: Vec<Medium>,
    pub materials: Vec<Bsdf>,
    pub shapes: Vec<Shape>,
    pub camera: Camera,
    pub film: Film,
}

#[derive(Debug, Clone)]
pub enum Bsdf {
    // Diffuse {
    //     reflectance: Colour,
    // },
}

#[derive(Debug, Clone)]
pub struct Object {
    pub shape: Shape,
    pub transform: Transform,
    // pub emitter: Option<Emitter>,
}

#[derive(Debug, Clone)]
pub enum Shape {

}

#[derive(Debug, Clone)]
pub struct Transform {

}

#[derive(Debug, Clone)]
pub enum Medium {}

#[derive(Debug, Clone)]
pub struct Camera {
    pub fov: f32,
}

#[derive(Debug, Clone)]
pub struct Film {
    width: u32,
    height: u32,
    samples: u32,
}