use math::*;

pub struct Quad {
    p0: Point3<f32>,
    p1: Point3<f32>,
    p2: Point3<f32>,
    p3: Point3<f32>,
    pub emission: Colour,
}