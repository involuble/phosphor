use crate::math::*;
use embree;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub tfar: f32,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3, tfar: f32) -> Self {
        debug_assert!(tfar > 0.0, "Invalid tfar");
        Ray {
            origin: origin,
            dir: dir,
            tfar: tfar,
        }
    }

    // pub fn in_range(&self, t: f32) -> bool {
    //     t >= 0.0 && t < self.tfar
    // }

    pub fn point_at_dist(&self, t: f32) -> Vec3 {
        self.origin + t*self.dir
    }

    /// Offset the ray away from its origin. This is to avoid precision issues
    pub fn offset(&mut self, offset_dir: Vec3) {
        self.origin += offset_dir * EPSILON;
    }
}

impl Into<embree::Ray> for Ray {
    fn into(self) -> embree::Ray {
        embree::Ray::new(self.origin, self.dir, 0.0, self.tfar)
    }
}