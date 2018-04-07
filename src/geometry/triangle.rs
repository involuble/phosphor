use cgmath::*;

use intersection::*;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p1: Point3<f32>,
    pub p2: Point3<f32>,
    pub p3: Point3<f32>,
}

impl Triangle {
    pub fn new(p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) -> Self {
        Triangle { p1: p1, p2: p2, p3: p3 }
    }
}

impl Intersectable for Triangle {
    // Reference: [MollerTrumbore97]
    //    http://www.graphics.cornell.edu/pubs/1997/MT97.html
    // TODO: See also http://jcgt.org/published/0002/01/05/paper.pdf for watertight intersections
    fn intersect(&self, ray: &Ray) -> Intersection {
        let e1 = self.p2 - self.p1;
        let e2 = self.p3 - self.p1;
        
        let pvec = ray.dir.cross(e2);
        let det = dot(e1, pvec);

        if relative_eq!(det, 0.0, epsilon=EPSILON) {
            return Intersection::miss();
        }

        let inv_det = 1.0 / det;
        let tvec = ray.origin - self.p1;
        let u = dot(tvec, pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return Intersection::miss();
        }

        let qvec = tvec.cross(e1);
        let v = dot(ray.dir, qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return Intersection::miss();
        }

        let t = dot(e2, qvec) * inv_det;

        if t < 0.0 {
            return Intersection::miss();
        }

        let mut n = e2.cross(e1).normalize();
        if dot(n, ray.dir) > 0.0 {
            n = -n;
        }
        Intersection::hit(ray.origin + t*ray.dir, t, n, u, v)
    }
}