use std::f32;

use primitives::*;

#[derive(Debug)]
pub struct TriangleList {
    pub triangles: Vec<Triangle>,
}

impl TriangleList {
    pub fn from_vec(tris: Vec<Triangle>) -> Self {
        TriangleList {
            triangles: tris,
        }
    }
}

impl Intersectable for TriangleList {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut hit = None;
        for tri in &self.triangles {
            let new_hit = tri.intersect(&ray);
            Intersection::replace_closest(&mut hit, &new_hit, EPSILON);
        }
        hit
    }
}
