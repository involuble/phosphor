use std::f32::consts::PI;
use na::*;

use primitive::*;

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