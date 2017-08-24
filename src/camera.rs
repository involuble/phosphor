use na::*;
use std::f32::consts::{PI};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub loc: Point3<f32>,
    pub forward: Vector3<f32>,
    pub up: Vector3<f32>,
    pub right: Vector3<f32>,
    pub fov: f32,
}

impl Camera {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Camera {
            loc: Point3::origin(),
            forward: -Vector3::z(),
            up: Vector3::y(),
            right: Vector3::x(),
            fov: PI / 2.0,
        }
    }

    // TODO: Probably make a CameraBuilder type for this
    pub fn new(l: Point3<f32>, forward: Vector3<f32>, up: Vector3<f32>, fov_degrees: f32) -> Self {
        Camera { loc: l, forward: forward, up: up, right: forward.cross(&up), fov: fov_degrees.to_radians() }
    }
}