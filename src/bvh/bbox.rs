use cgmath::*;

use geometry::*;

pub struct BBox {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

pub trait Boundable {
    fn bounds(&self) -> BBox;
}

impl BBox {
    // https://tavianator.com/fast-branchless-raybounding-box-intersections-part-2-nans/
    pub fn intersect(&self, ray: Ray) -> bool {
        let inv_dir = Vector3::new(1.0 / ray.dir.x, 1.0 / ray.dir.y, 1.0 / ray.dir.z);
        
        let t1 = (self.min[0] - ray.origin[0]) * inv_dir[0];
        let t2 = (self.max[0] - ray.origin[0]) * inv_dir[0];

        let mut tmin = min(t1, t2);
        let mut tmax = max(t1, t2);

        for i in 1..3 {
            let t1 = (self.min[i] - ray.origin[i]) * inv_dir[i];
            let t2 = (self.max[i] - ray.origin[i]) * inv_dir[i];

            tmin = max(tmin, min(t1, t2));
            tmax = min(tmax, max(t1, t2));
        }

        tmax >= max(tmin, 0.0)
    }

    pub fn union(b1: BBox, b2: BBox) -> BBox {
        let min = Vector3::new(min(b1[0], b2[0]), min(b1[1], b2[1]), min(b1[2], b2[2]));
        let max = Vector3::new(max(b1[0], b2[0]), max(b1[1], b2[1]), max(b1[2], b2[2]));
        BBox { min: min, max: max }
    }

    // pub fn max_dim(&self) -> u32 {
    //     let extent = self.max - self.min;
    //     extent.max_dim();
    // }

    pub fn surface_area(&self) -> f32 {
        let extent = self.max - self.min;
        2.0 * extent.norm_sq()
    }
}