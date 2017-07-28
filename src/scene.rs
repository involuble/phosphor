use primitive::*;

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
    pub tris: Vec<Triangle>,
    pub spheres: Vec<Sphere>,
    pub camera: Camera,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            tris: Vec::new(),
            spheres: Vec::new(),
            camera: Camera {
                loc: Point3::origin(),
                forward: -Vector3::z(),
                up: Vector3::y(),
                fov: PI/2.0,
            }
        }
    }
}