use cgmath::*;
use num_traits::{Zero};
use std::f32;

pub const EPSILON: f32 = 1e-5;

pub const INVALID_GEOM_ID: u32 = !0;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub dir: Vector3<f32>,
}

impl Ray {
    pub fn new(o: Point3<f32>, dir: Vector3<f32>) -> Self {
        Ray {origin: o, dir: dir }
    }
}

// Note: The lifetime declarations if a reference to the primitive is desired instead of an id
// pub struct Intersection<'a, T: 'a> {
//     pub geom: &'a T,

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub hit: bool,
    pub p: Point3<f32>,
    pub t: f32,
    // The geometric normal (normalized)
    pub n: Vector3<f32>,
    pub u: f32,
    pub v: f32,
    pub geom_id: u32,
}

impl Intersection {
    pub fn hit(p: Point3<f32>, t: f32, n: Vector3<f32>, u: f32, v: f32) -> Self {
        Intersection { hit: true, t: t, p: p, n: n, u: u, v: v, geom_id: INVALID_GEOM_ID }
    }

    pub fn miss() -> Self {
        Intersection { hit: false, p: Point3::origin(), t: f32::INFINITY, n: Vector3::zero(),
            u: 0.0, v: 0.0, geom_id: INVALID_GEOM_ID }
    }

    pub fn replace_closest(lhs: &mut Intersection, rhs: &Intersection, min: f32) {
        if rhs.t < lhs.t && rhs.t > min {
            *lhs = *rhs;
        }
    }

    pub fn set_geom_id(i: &mut Intersection, id: u32) {
        i.geom_id = id;
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Intersection;
}