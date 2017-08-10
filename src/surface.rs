use na::*;

use primitives::*;
use materials::*;

#[derive(Debug, Clone, Copy)]
pub struct SurfaceInfo {
    pub n_shading: Vector3<f32>,
    pub material: Material,
}

pub trait Geometry: Intersectable {
    fn get_surface_info(&self, i: &Intersection) -> SurfaceInfo;
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub material: Material,
    pub geom_id: u32,
}

impl Sphere {
    pub fn new(c: Point3<f32>, r: f32, mat: Material) -> Self {
        Sphere { center: c, radius: r, material: mat, geom_id: INVALID_GEOM_ID }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let s = SpherePrimitive::new(self.center, self.radius);
        let mut o = s.intersect(ray);
        match o.as_mut() {
            Some(i) => i.geom_id = self.geom_id,
            _ => (),
        };
        o
    }
}

impl Geometry for Sphere {
    fn get_surface_info(&self, i: &Intersection) -> SurfaceInfo {
        SurfaceInfo { n_shading: i.n, material: self.material }
    }
}