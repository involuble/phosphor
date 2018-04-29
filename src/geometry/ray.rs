use math::*;
use embree;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub dir: Vector3<f32>,
    pub tfar: f32,
}

impl Ray {
    pub fn new(origin: Point3<f32>, dir: Vector3<f32>, tfar: f32) -> Self {
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

    pub fn point_at_dist(&self, t: f32) -> Point3<f32> {
        self.origin + t*self.dir
    }

    // Offset the ray away from its origin. This is to avoid precision issues
    pub fn offset_origin(&mut self, offset_dir: Vector3<f32>) {
        unimplemented!()
    }
}

impl Into<embree::Ray> for Ray {
    fn into(self) -> embree::Ray {
        embree::Ray::new(self.origin, self.dir, 0.0, self.tfar)
    }
}