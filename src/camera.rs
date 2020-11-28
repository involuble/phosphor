use std::f32;
use crate::math::prelude::*;

use scene_import;

use crate::geometry::*;

pub struct Camera {
    pub origin: Vec3,
    forward: Vec3,
    down: Vec3,
    right: Vec3,
    upper_left: Vec3,
}

impl From<scene_import::Camera> for Camera {
    fn from(camera: scene_import::Camera) -> Self {
        let res = camera.resolution;
        let aspect = (res[0] as f32) / (res[1] as f32);
        Camera::new(
            camera.transform.position.into(),
            camera.transform.look_at.into(),
            camera.transform.up.into(),
            camera.fov_degrees,
            aspect
        )
    }
}

impl Camera {
    pub fn new(origin: Vec3, look_at: Vec3, up: Vec3, fov_vertical_degrees: f32, aspect_ratio: f32) -> Self {
        let forward = (look_at - origin).normalize();
        let fov: f32 = fov_vertical_degrees * PI / 180.0;
        let half_height = (fov / 2.0).tan();
        let half_width  = half_height * aspect_ratio;
        let right = forward.cross(up).normalize();
        let down = forward.cross(right).normalize();
        Camera {
            origin: origin,
            forward: forward,
            down: 2.0 * half_height * down,
            right: 2.0 * half_width * right,
            upper_left: -half_width * right + -half_height * down,
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let dir = self.upper_left + x * self.right + y * self.down + self.forward;
        Ray::new(self.origin, dir.normalize(), f32::MAX)
    }
}