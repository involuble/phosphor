use surface::*;
use material::*;
use geometry::*;

#[derive(Debug)]
pub struct Mesh {
    pub tris: IntersectableList<Triangle>,
    pub material: Material,
    pub geom_id: u32,
}

impl Mesh {
    pub fn new(tris: Vec<Triangle>, mat: Material) -> Self {
        Mesh { tris: IntersectableList::build(tris), material: mat, geom_id: INVALID_GEOM_ID }
    }
}

impl Surface for Mesh {
    fn get_surface_info(&self, i: &Intersection) -> SurfaceInfo {
        SurfaceInfo { n_shading: i.n, material: self.material }
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let mut o = self.tris.intersect(ray);
        Intersection::set_geom_id(&mut o, self.geom_id);
        o
    }
}