use math::*;
use colour::*;
// use geometry::{AreaLight, LightSample};

pub struct Quad {
    p0: Point3<f32>,
    p1: Point3<f32>,
    p2: Point3<f32>,
    p3: Point3<f32>,
    pub emission: Colour,
}

impl Quad {
    pub fn new(p0: Point3<f32>, p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) -> Quad {
        Quad {
            p0: p0,
            p1: p1,
            p2: p2,
            p3: p3,
            emission: Colour::zero(),
        }
    }

    pub fn is_emissive(&self) -> bool {
        !self.emission.is_zero()
    }

    pub fn points(&self) -> [Point3<f32>; 4] {
        [self.p0, self.p1, self.p2, self.p3]
    }
}

impl Transformable for Quad {
    fn transform_by(&mut self, transform: &AffineTransform) {
        self.p0 = transform.transform_point(self.p0);
        self.p1 = transform.transform_point(self.p1);
        self.p2 = transform.transform_point(self.p2);
        self.p3 = transform.transform_point(self.p3);
    }
}