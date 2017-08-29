use na::*;
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

#[derive(Debug, Clone, Copy)]
pub struct SpherePrimitive {
    pub center: Point3<f32>,
    pub radius: f32,
}

impl SpherePrimitive {
    pub fn new(c: Point3<f32>, r: f32) -> Self {
        SpherePrimitive { center: c, radius: r }
    }
}

// Note: The lifetime declarations if a reference to the primitive is desired instead of an id
// pub struct Intersection<'a, T: 'a> {
//     pub prim: &'a T,

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

impl Intersectable for SpherePrimitive {
    // See https://en.wikipedia.org/wiki/Line-sphere_intersection
    fn intersect(&self, ray: &Ray) -> Intersection {
        let a = self.center - ray.origin;
        let adj = dot(&a, &ray.dir);
        let det = adj*adj - dot(&a,&a) + self.radius*self.radius;
        if det < 0.0 {
            return Intersection::miss();
        }
        let sdet = det.sqrt();
        let s1 = adj + sdet;
        let s2 = adj - sdet;
        let dist;
        if s2 < s1 && s2 > EPSILON { dist = s2; }
        else if s1 > EPSILON { dist = s1; }
        else { return Intersection::miss(); }

        let p = ray.origin + ray.dir * dist;
        let n = (p - self.center)/self.radius;

        Intersection::hit(p, dist, n, 0.0, 0.0)
    }
}

impl Intersectable for Triangle {
    // Reference: [MollerTrumbore97]
    //    http://www.graphics.cornell.edu/pubs/1997/MT97.html
    // TODO: See also http://jcgt.org/published/0002/01/05/paper.pdf for watertight intersections
    fn intersect(&self, ray: &Ray) -> Intersection {
        let e1 = self.p2 - self.p1;
        let e2 = self.p3 - self.p1;
        
        let pvec = ray.dir.cross(&e2);
        let det = dot(&e1, &pvec);

        if relative_eq!(det, 0.0, epsilon=EPSILON) {
            return Intersection::miss();
        }

        let inv_det = 1.0 / det;
        let tvec = ray.origin - self.p1;
        let u = dot(&tvec, &pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return Intersection::miss();
        }

        let qvec = tvec.cross(&e1);
        let v = dot(&ray.dir, &qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return Intersection::miss();
        }

        let t = dot(&e2, &qvec) * inv_det;

        if t < 0.0 {
            return Intersection::miss();
        }

        let mut n = normalize(&e2.cross(&e1));
        if dot(&n, &ray.dir) > 0.0 {
            n = -n;
        }
        Intersection::hit(ray.origin + t*ray.dir, t, n, u, v)
    }
}
