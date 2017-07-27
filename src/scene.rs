use std::f32::consts::PI;
use na::*;

use primitive::*;

pub struct Scene {
    pub prims: Vec<Sphere>,
    pub camera: Camera,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            prims: Vec::new(),
            camera: Camera {
                loc: Point3::origin(),
                look_at: -Vector3::z(),
                up: Vector3::y(),
                fov: PI/2.0,
            }
        }
    }
}