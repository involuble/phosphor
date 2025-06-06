#![allow(dead_code)]

use std::f32;
use std::f32::consts::PI;

use glam::*;
use embree::*;
use derive_more::*;

pub struct Camera {
    pub origin: Vec3,
    forward: Vec3,
    down: Vec3,
    right: Vec3,
    upper_left: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, look_at: Vec3, up: Vec3, fov_degrees: f32, aspect_ratio: f32) -> Self {
        let forward = (look_at - origin).normalize();
        let fov: f32 = fov_degrees * PI / 180.0;
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
        Ray::new(self.origin, dir.normalize(), 0.0, f32::MAX)
    }
}

pub struct UserSphere {
    pub center: Vec3,
    pub radius: f32,
}

impl UserPrimitive for UserSphere {
    fn intersect(&self, ray: &Ray) -> UserPrimHit {
        let v = ray.origin - self.center;

        let a = ray.dir.length_squared();
        let b = 2.0 * v.dot(ray.dir);
        let c = v.length_squared() - self.radius * self.radius;
        let d = b*b - 4.0 * a * c;
        if d < 0.0 {
            return UserPrimHit::miss()
        }

        let q = d.sqrt();
        let rcp_a = 1.0 / a;

        let t0 = 0.5 * rcp_a * (-b - q);
        if ray.in_range(t0) {
            return UserPrimHit {
                t: t0,
                Ng: ray.point_at_dist(t0) - self.center,
                uv: Vec2::ZERO,
            }
        }
        let t1 = 0.5 * rcp_a * (-b + q);
        if ray.in_range(t1) {
            return UserPrimHit {
                t: t1,
                Ng: ray.point_at_dist(t1) - self.center,
                uv: Vec2::ZERO,
            }
        }
        UserPrimHit::miss()
    }

    fn bounds(&self) -> Bounds {
        Bounds::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius))
    }
}

#[derive(Debug, Copy, Clone, Constructor, Add, Mul, AddAssign)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Colour {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }

    pub fn to_rgba8(&self) -> u32 {
        let r: u32 = (self.r.powf(1.0/2.2) * 255.0).clamp(0.0, 255.0) as u32;
        let g: u32 = (self.g.powf(1.0/2.2) * 255.0).clamp(0.0, 255.0) as u32;
        let b: u32 = (self.b.powf(1.0/2.2) * 255.0).clamp(0.0, 255.0) as u32;
        0xFF000000 | r << 16 | g << 8 | b
    }
}
