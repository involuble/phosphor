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
        let mut dist = f32::INFINITY;
        for tri in &self.triangles {
            let new_hit = tri.intersect(&ray);
            let new_dist = Intersection::get_dist(&new_hit);
            if new_dist < dist && new_dist > EPSILON {
                hit = new_hit;
                dist = new_dist;
            }
        }
        hit
    }
}
