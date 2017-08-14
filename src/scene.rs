use colour::*;
use primitives::*;
use material::*;
use surface::*;
use mesh::*;

pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Sphere>,
    pub background: Colour,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            meshes: Vec::new(),
            spheres: Vec::new(),
            lights: Vec::new(),
            background: Colour::black(),
        }
    }

    pub fn add_sphere(&mut self, mut sphere: Sphere) {
        sphere.geom_id = (self.spheres.len() + 200) as u32;
        self.spheres.push(sphere);
    }

    pub fn add_mesh(&mut self, list: Vec<Triangle>, mat: Material) {
        let mut mesh = Mesh::new(list, mat);
        mesh.geom_id = self.meshes.len() as u32;
        self.meshes.push(mesh);
    }

    pub fn get_surface_info(&self, geom_id: u32, i: &Intersection) -> SurfaceInfo {
        if geom_id >= 200 {
            let s = &self.spheres[(geom_id - 200) as usize];
            s.get_surface_info(i)
        } else {
            let m = &self.meshes[geom_id as usize];
            m.get_surface_info(i)
        }
    }

    pub fn add_light(&mut self, mut light: Sphere) {
        let e = light.material.emittance;
        assert!(!e.is_black());
        let id = self.spheres.len() as u32 + 200;
        light.geom_id = id;
        self.lights.push(light);
        self.spheres.push(light);
    }
}
