use crate::math::*;
use embree::{Ray, UserPrimHit, UserPrimitive, Bounds};

#[derive(Debug, Clone)]
pub struct OrientedBox {
    center: Point3<f32>,
    radius: f32,
    // rotation: Quaternion<f32>,
}

impl Transformable for OrientedBox {
    fn transform_by(&mut self, transform: &AffineTransform) {
        if !transform.is_similarity() {
            log::warn!("Can't transform box by non-uniform scale");
        }
        self.center = transform.transform_point(self.center);
        self.radius *= transform.scale.x;

        todo!();
    }
}

impl UserPrimitive for OrientedBox {
    fn intersect(&self, _ray: &Ray) -> UserPrimHit {
        // See "A Ray-Box Intersection Algorithm and Efficient Dynamic Voxel Rendering"
        //  from JCGT
        // let mut ray = ray.clone();
        // let inv_dir = Vector3::new(1.0 / ray.dir.x, 1.0 / ray.dir.y, 1.0 / ray.dir.z);
        // ray.origin = ray.origin - self.center.into();

        // let abs = ray.origin.abs();
        // let extent = max(max(abs.x, abs.y), abs.z) / self.radius;
        // let winding = if extent < 1.0 { -1.0 } else { 1.0 };
        
        // let sgn = Vector3::new(-ray.dir.x.signum(), -ray.dir.y.signum(), -ray.dir.z.signum());
        // let d = self.radius * winding * sgn - ray.origin;
        // d = d * inv_dir;

        todo!()
    }

    fn bounds(&self) -> Bounds {
        todo!()
    }
}