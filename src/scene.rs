use primitive::*;
use material::*;

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
    pub tris: Vec<Triangle>,
    pub spheres: Vec<Sphere>,
    pub materials: Vec<Material>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            camera: Camera {
                loc: Point3::origin(),
                forward: -Vector3::z(),
                up: Vector3::y(),
                fov: PI/2.0,
            },
            tris: Vec::new(),
            spheres: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        assert!(self.materials.len() as u32 > sphere.material_id, "Invalid material ID");
        self.spheres.push(sphere);
    }

    pub fn add_triangle(&mut self, triangle: Triangle) {
        assert!(self.materials.len() as u32 > triangle.material_id, "Invalid material ID");
        self.tris.push(triangle);
    }

    pub fn add_material(&mut self, mut material: Material) {
        let idx = self.materials.len();
        material.set_id(idx as u32);
        self.materials.push(material);
    }
}