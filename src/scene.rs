use primitives::*;
use materials::*;
use triangle_list::*;
use lights::*;

pub struct Scene {
    pub tri_lists: Vec<TriangleList>,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<SphereLight>,
    // TODO: Keep using material IDs or switch to references?
    materials: Vec<Material>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            tri_lists: Vec::new(),
            spheres: Vec::new(),
            lights: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        assert!(
            self.materials.len() as u32 > sphere.material_id,
            "Invalid material ID"
        );
        self.spheres.push(sphere);
    }

    pub fn add_triangle_list(&mut self, list: TriangleList) {
        assert!(
            self.materials.len() as u32 > list.material_id,
            "Invalid material ID"
        );
        self.tri_lists.push(list);
    }

    pub fn add_material(&mut self, mut material: Material) {
        let idx = self.materials.len();
        material.set_id(idx as u32);
        self.materials.push(material);
    }

    pub fn get_material(&self, id: u32) -> &Material {
        &self.materials[id as usize]
    }

    pub fn add_light(&mut self, light: Sphere) {
        let e = self.get_material(light.material_id).emittance;
        debug_assert!(!e.is_black());
        self.lights.push(SphereLight { sphere: light, emittance: e });
        self.spheres.push(light);
    }
}
