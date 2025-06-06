use std::ffi::c_void;
use std::ptr;
use std::u32;
use std::f32;

use glam::*;

use sys::*;

use crate::common::*;
use crate::device::*;
use crate::geometry::*;
use crate::ray::*;

pub trait UserPrimitive: 'static + Send + Sync {
    fn intersect(&self, ray: &Ray) -> UserPrimHit;
    fn bounds(&self) -> Bounds;

    // TODO: Maybe expose this for convenience?
    // fn transform_by(&mut self, mat: Mat4);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct UserPrimHit {
    pub t: f32,
    pub Ng: Vec3,
    pub uv: Vec2,
}

impl UserPrimHit {
    pub fn new(t: f32, normal: Vec3, uv: Vec2) -> Self {
        UserPrimHit {
            t: t,
            Ng: normal,
            uv: uv,
        }
    }

    pub fn miss() -> Self {
        UserPrimHit {
            t: f32::MIN,
            Ng: Vec3::ZERO,
            uv: Vec2::ZERO,
        }
    }
}

pub struct UserGeometry<T> {
    handle: GeometryHandle,
    pub prims: Vec<T>,
}

impl<T: UserPrimitive> UserGeometry<T> {
    pub fn new(device: &Device, prims: Vec<T>) -> Self {
        let handle = GeometryHandle::new(device, GeometryType::User);
        UserGeometry {
            handle,
            prims,
        }
    }
}

impl<T: UserPrimitive> Geometry for UserGeometry<T> {
    fn handle(&self) -> &GeometryHandle {
        &self.handle
    }

    fn handle_mut(&mut self) -> &mut GeometryHandle {
        &mut self.handle
    }

    fn bind_buffers(&mut self) {
        // TODO: Taking Pin<&mut self> would guarantee the SetUserData call is safe
        assert!(self.prims.len() <= u32::MAX as usize);

        // Pass self as user_ptr
        // let user_ptr = self as *const UserGeometry<T> as *mut c_void;
        let user_ptr = self.prims.as_ptr() as *mut c_void;
        unsafe {
            let handle = self.handle.as_raw_ptr();
            rtcSetGeometryUserPrimitiveCount(handle, self.prims.len() as u32);
            rtcSetGeometryUserData(handle, user_ptr);
            rtcSetGeometryBoundsFunction(handle, Some(bounds_func::<T>), user_ptr);
            rtcSetGeometryIntersectFunction(handle, Some(intersect_func::<T>));
            rtcSetGeometryOccludedFunction(handle, Some(occluded_func::<T>));
        }
    }
}

unsafe extern "C" fn bounds_func<T: UserPrimitive>(args: *const RTCBoundsFunctionArguments) {
    // userPtr is &[T]:
    let data_ptr = (*args).geometryUserPtr as *const T;
    let prim_ptr = data_ptr.offset((*args).primID as isize);
    let prim: &T = prim_ptr.as_ref().unwrap();

    // userPtr is &UserGeometry<T>:
    // let geometry: &UserGeometry<T> = ((*args).geometryUserPtr as *const UserGeometry<T>).as_ref().unwrap();
    // let prim: &T = &geometry.prims[(*args).primID as usize];

    let bounds = prim.bounds();
    debug_assert!(bounds.lower.x <= bounds.upper.x);
    ptr::write((*args).bounds_o as *mut Bounds, bounds);
}

unsafe extern "C" fn intersect_func<T: UserPrimitive>(args: *const RTCIntersectFunctionNArguments) {
    let data_ptr = (*args).geometryUserPtr as *const T;
    let prim_ptr = data_ptr.offset((*args).primID as isize);
    let prim: &T = prim_ptr.as_ref().unwrap();

    debug_assert!((*args).N == 1);
    if *(*args).valid == 0 { return; }

    let rayhit = (*args).rayhit as *mut RTCRayHit as *mut RayHit;
    let rayhit = &mut *rayhit;

    let ray = &mut rayhit.ray;
    let hit = &mut rayhit.hit;

    // TODO: need to expose a way to call rtcFilterIntersection (possibly by passing a closure in)
    // TODO: catch_unwind
    let prim_hit = prim.intersect(ray);

    if prim_hit.t >= ray.tnear {
        // The UserPrimitive intersect function should make sure the below invariant holds
        //  but check it anyways. This could be turned into a runtime check instead of an assert
        debug_assert!(ray.in_range(prim_hit.t), "Intersect function returning distance out of ray bounds");
        ray.tfar = prim_hit.t;
        hit.Ng = prim_hit.Ng;
        hit.uv = prim_hit.uv;
        hit.prim_id = (*args).primID.into();
        hit.geom_id = (*args).geomID.into();
        hit.inst_id = (*(*args).context).instID[0].into();
    }
}

unsafe extern "C" fn occluded_func<T: UserPrimitive>(args: *const RTCOccludedFunctionNArguments) {
    let data_ptr = (*args).geometryUserPtr as *const T;
    let prim_ptr = data_ptr.offset((*args).primID as isize);
    let prim: &T = prim_ptr.as_ref().unwrap();

    debug_assert!((*args).N == 1);
    if *(*args).valid == 0 { return; }

    let ray = (*args).ray as *mut RTCRay as *mut Ray;
    let ray = &mut *ray;

    let prim_hit = prim.intersect(ray);

    if prim_hit.t >= ray.tnear && prim_hit.t <= ray.tfar {
        ray.tfar = std::f32::NEG_INFINITY;
    }
}