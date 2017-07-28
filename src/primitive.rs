use na::*;
use std::f32;

type UnitVector3<T> = Unit<Vector3<T>>;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub dir: Unit<Vector3<f32>>,
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
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(c: Point3<f32>, r: f32) -> Self {
        Sphere { center: c, radius: r }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub p: Point3<f32>,
    pub n: UnitVector3<f32>,
    pub d: f32,
    pub u: f32,
    pub v: f32,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

impl Intersectable for Sphere {
    // See https://en.wikipedia.org/wiki/Line-sphere_intersection
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let a = self.center - ray.origin;
        let adj = dot(&a, ray.dir.as_ref());
        let det = adj*adj - dot(&a,&a) + self.radius*self.radius;
        if det < 0.0 {
            return None;
        }
        let sdet = det.sqrt();
        let s1 = adj + sdet;
        let s2 = adj - sdet;
        let dist;
        if s2 < s1 && s2 > f32::EPSILON { dist = s2; }
        else if s1 > f32::EPSILON { dist = s1; }
        else { return None; }

        let p = ray.origin + ray.dir.unwrap() * dist;
        let n = (p - self.center)/self.radius;

        // TODO
        let u = 0.0;
        let v = 0.0;
        Some(Intersection {p: p, n: Unit::new_unchecked(n), d: dist, u: u, v: v })
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let p = self.p1;
        let u = self.p2 - self.p1;
        let v = self.p3 - self.p1;

        let n = u.cross(&v);
        let b = dot(&n, ray.dir.as_ref());

        let to_ray_origin = ray.origin - p;
        // distance to the plane that the triangle lies on
        let dist = dot(&n, &to_ray_origin) / -b;

        if relative_eq!(b, 0.0) || dist < f32::EPSILON {
            return None;
        }

        // Intersection of the ray and the plane the triangle lies on (relative to the triangle center)
        let w = to_ray_origin + ray.dir.unwrap() * dist;

        let uu = dot(&u, &u);
        let vv = dot(&v, &v);
        let uv = dot(&u, &v);
        let wv = dot(&w, &v);
        let wu = dot(&w, &u);

        let inv_den = 1.0 / (uv*uv - uu*vv);
        // Barycentric coordinates of w
        let s = (uv*wv - vv*wu) * inv_den;
        let t = (uv*wu - uu*wv) * inv_den;

        if s < 0.0 || t < 0.0 || (s+t) > 1.0 {
            return None;
        }

        Some(Intersection {p: Point3::from_coordinates(w), n: Unit::new_unchecked(n), d: dist, u: s, v: t})
    }
}