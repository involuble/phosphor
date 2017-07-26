use na::*;

pub struct Colour {
    pub g: f32,
    pub b: f32,
    pub r: f32,
}

impl Colour {
    pub fn from_luma(y: f32) -> Self {
        Colour { r: y, g: y, b: y }
    }

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Colour { r: r, g: g, b: b }
    }

    pub fn black() -> Self {
        Colour { r: 0.0, g: 0.0, b: 0.0 }
    }
}

pub struct Camera {
    pub loc: Point3<f32>,
    pub lookAt: Vector3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
}

pub struct Ray {
    pub origin: Point3<f32>,
    pub dir: Unit<Vector3<f32>>,
}

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(c: Point3<f32>, r: f32) -> Self {
        Sphere { center: c, radius: r }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    // See https://en.wikipedia.org/wiki/Line-sphere_intersection
    fn intersect(&self, ray: &Ray) -> bool {
        let a = self.center - ray.origin;
        let adj = dot(&a, ray.dir.as_ref());
        let det = adj*adj - dot(&a,&a) + self.radius*self.radius;
        if det < 0.0 {
            return false;
        }
        true
    }
}