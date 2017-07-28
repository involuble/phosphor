use std::f32;

use primitive::*;

pub struct TriangleList {
    pub triangles: Vec<Triangle>,
    pub material_id: u32,
}

impl TriangleList {
    pub fn new(mat: u32) -> Self {
        TriangleList {
            triangles: Vec::new(),
            material_id: mat,
        }
    }

    pub fn from_vec(tris: Vec<Triangle>, mat: u32) -> Self {
        TriangleList {
            triangles: tris,
            material_id: mat,
        }
    }
}

impl Traceable for TriangleList {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let f = |o: &Option<IntersectionGeometry>| o.map_or(f32::INFINITY, |i| i.d);
        let mut hit = None;
        for tri in &self.triangles {
            let new_hit = tri.intersect_geom(&ray);
            if f(&new_hit) < f(&hit) {
                hit = new_hit;
            }
        }
        match hit {
            Some(ig) => Some(Intersection {
                ig: ig,
                material_id: self.material_id,
            }),
            None => None,
        }
    }
}
