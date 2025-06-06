use std::u32;

use sys::*;

use glam::*;
#[cfg(test)]
use memoffset::*;

use crate::common::GeomID;

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
/// See https://www.embree.org/api.html#rtcray
pub struct Ray {
    /// Origin point of the ray
    pub origin: Vec3,
    /// Start of the ray segment. Must be >= 0
    pub tnear: f32,
    /// Direction of the ray
    pub dir: Vec3,
    /// The time associated with the ray (for motion blur)
    /// Always in the range [0,1]
    time: f32,
    /// End of ray segment. This field is set to the hit distance after an intersection query
    pub tfar: f32,
    /// Ray mask
    mask: u32,
    id: u32,
    flags: u32,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3, tnear: f32, tfar: f32) -> Self {
        debug_assert!(tnear >= 0.0, "Invalid tnear");
        debug_assert!(tfar > tnear, "Invalid tfar");
        Ray {
            origin: origin,
            tnear: tnear,
            dir: dir,
            time: 0.0,
            tfar: tfar,
            mask: u32::MAX,
            id: 0,
            flags: 0,
        }
    }

    pub fn in_range(&self, t: f32) -> bool {
        t >= self.tnear && t <= self.tfar
    }

    pub fn point_at_dist(&self, t: f32) -> Vec3 {
        self.origin + t*self.dir
    }
}

#[test]
fn test_ray_layout() {
    assert_eq!(std::mem::size_of::<Ray>(), std::mem::size_of::<RTCRay>());
    assert_eq!(std::mem::align_of::<Ray>(), std::mem::align_of::<RTCRay>());
    assert_eq!(offset_of!(Ray, origin), offset_of!(RTCRay, org_x));
    assert_eq!(offset_of!(Ray, tnear), offset_of!(RTCRay, tnear));
    assert_eq!(offset_of!(Ray, dir), offset_of!(RTCRay, dir_x));
    assert_eq!(offset_of!(Ray, time), offset_of!(RTCRay, time));
    assert_eq!(offset_of!(Ray, tfar), offset_of!(RTCRay, tfar));
    assert_eq!(offset_of!(Ray, mask), offset_of!(RTCRay, mask));
    assert_eq!(offset_of!(Ray, id), offset_of!(RTCRay, id));
    assert_eq!(offset_of!(Ray, flags), offset_of!(RTCRay, flags));
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
/// See https://www.embree.org/api.html#rtchit
pub struct Hit {
    pub Ng: Vec3,
    pub uv: Vec2,
    pub prim_id: GeomID,
    pub geom_id: GeomID,
    pub inst_id: GeomID,
}

impl Hit {
    pub fn empty() -> Self {
        Hit {
            Ng: Vec3::ZERO,
            uv: Vec2::ZERO,
            geom_id: GeomID::invalid(),
            prim_id: GeomID::invalid(),
            inst_id: GeomID::invalid(),
        }
    }

    pub fn is_hit(&self) -> bool {
        !self.geom_id.is_invalid()
    }
}

#[test]
fn test_hit_layout() {
    assert_eq!(std::mem::size_of::<Hit>(), std::mem::size_of::<RTCHit>());
    assert_eq!(std::mem::align_of::<Hit>(), std::mem::align_of::<RTCHit>());
    assert_eq!(offset_of!(Hit, Ng), offset_of!(RTCHit, Ng_x));
    assert_eq!(offset_of!(Hit, uv), offset_of!(RTCHit, u));
    assert_eq!(offset_of!(Hit, prim_id), offset_of!(RTCHit, primID));
    assert_eq!(offset_of!(Hit, geom_id), offset_of!(RTCHit, geomID));
    assert_eq!(offset_of!(Hit, inst_id), offset_of!(RTCHit, instID));
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct RayHit {
    pub ray: Ray,
    pub hit: Hit,
}

impl RayHit {
    pub fn from_ray(ray: Ray) -> Self {
        RayHit { ray: ray, hit: Hit::empty() }
    }
    
    pub fn as_raw_ptr(&mut self) -> *mut RTCRayHit {
        self as *mut RayHit as *mut RTCRayHit
    }
}
