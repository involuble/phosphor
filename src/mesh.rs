use surface::*;
use materials::*;
use primitives::*;
use triangle_list::*;

#[derive(Debug)]
pub struct Mesh {
    pub tris: TriangleList,
    pub material: Material,
    pub geom_id: u32,
}

impl Mesh {
    pub fn new(tris: Vec<Triangle>, mat: Material) -> Self {
        Mesh { tris: TriangleList::from_vec(tris), material: mat, geom_id: INVALID_GEOM_ID }
    }
}

impl Geometry for Mesh {
    fn get_surface_info(&self, i: &Intersection) -> SurfaceInfo {
        SurfaceInfo { n_shading: i.n, material: self.material }
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut o = self.tris.intersect(ray);
        match o.as_mut() {
            Some(i) => i.geom_id = self.geom_id,
            _ => (),
        };
        o
    }
}