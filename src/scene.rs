use primitive::*;
use material::*;
use triangle_list::*;

use std::f32::consts::PI;
use na::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub loc: Point3<f32>,
    pub forward: Vector3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
}

pub struct Scene {
    pub camera: Camera,
    pub tri_lists: Vec<TriangleList>,
    pub spheres: Vec<Sphere>,
    materials: Vec<Material>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            camera: Camera {
                loc: Point3::origin(),
                forward: -Vector3::z(),
                up: Vector3::y(),
                fov: PI / 2.0,
            },
            tri_lists: Vec::new(),
            spheres: Vec::new(),
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
}
