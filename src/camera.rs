use na::*;
use std::f32::consts::{PI};

use geometry::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub loc: Point3<f32>,
    pub forward: Vector3<f32>,
    pub up: Vector3<f32>,
    pub right: Vector3<f32>,
    pub fov: f32,
    pub fov_scale: f32,
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
            fov_scale: 1.0,
        }
    }

    // TODO: Perhaps make a CameraBuilder type for this
    pub fn new(l: Point3<f32>, forward: Vector3<f32>, up: Vector3<f32>, fov_degrees: f32) -> Self {
        let fov_rad = fov_degrees.to_radians();
        let fov_scale = (fov_rad / 2.0).tan();
        Camera { loc: l, forward: forward, up: up, right: forward.cross(&up).normalize(), fov: fov_rad, fov_scale: fov_scale }
    }

    pub fn camera_ray_from_ss_coords(&self, coords: Vector2<f32>) -> Ray {
        let dir = self.forward + coords.x*self.right + coords.y*self.up;
        let dir_n = dir.normalize();
        Ray { origin: self.loc, dir: dir_n }
    }
}