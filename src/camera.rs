use std::f32;
use crate::math::prelude::*;

use crate::geometry::*;

pub struct Camera {
    pub origin: Point3<f32>,
    forward: Vector3<f32>,
    down: Vector3<f32>,
    right: Vector3<f32>,
    upper_left: Vector3<f32>,
}

impl Camera {
    pub fn new(origin: Point3<f32>, look_at: Point3<f32>, up: Vector3<f32>, fov_vertical: Deg<f32>, aspect_ratio: f32) -> Self {
        let forward = (look_at - origin).normalize();
        let fov: Rad<f32> = fov_vertical.into();
        let half_height = (fov.0 / 2.0).tan();
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