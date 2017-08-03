use na::*;
use std::f32;

// type UnitVector3<T> = Unit<Vector3<T>>;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub dir: Unit<Vector3<f32>>,
}

impl Ray {
    pub fn new(o: Point3<f32>, dir: Unit<Vector3<f32>>) -> Self {
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
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub material_id: u32,
}

impl Sphere {
    pub fn new(c: Point3<f32>, r: f32, mat: u32) -> Self {
        Sphere { center: c, radius: r, material_id: mat }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SurfaceIntersection {
    pub prim_i: PrimitiveIntersection,
    pub material_id: u32,
}

impl SurfaceIntersection {
    pub fn get_dist(o: &Option<SurfaceIntersection>) -> f32 {
        o.map_or(f32::INFINITY, |i| i.prim_i.d)
    }
}

pub trait Surface {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceIntersection>;
}

impl Surface for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceIntersection> {
        let i = self.intersect_prim(ray);
        if let Some(prim_i) = i {
            return Some(SurfaceIntersection {prim_i: prim_i, material_id: self.material_id});
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PrimitiveIntersection {
    pub p: Point3<f32>,
    pub n: Vector3<f32>,
    pub tang: Vector3<f32>,
    pub d: f32,
    pub u: f32,
    pub v: f32,
}

pub trait Primitive {
    fn intersect_prim(&self, ray: &Ray) -> Option<PrimitiveIntersection>;
}

impl Primitive for Sphere {
    // See https://en.wikipedia.org/wiki/Line-sphere_intersection
    fn intersect_prim(&self, ray: &Ray) -> Option<PrimitiveIntersection> {
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

        let tang = Vector3::new(0.0, 0.0, 0.0);
        Some(PrimitiveIntersection {p: p, n: n, tang: tang, d: dist, u: u, v: v})
    }
}

impl Primitive for Triangle {
    // http://www.graphics.cornell.edu/pubs/1997/MT97.html
    // TODO: See also http://jcgt.org/published/0002/01/05/paper.pdf for watertight intersections
    fn intersect_prim(&self, ray: &Ray) -> Option<PrimitiveIntersection> {
        let e1 = self.p2 - self.p1;
        let e2 = self.p3 - self.p1;

        let pvec = ray.dir.cross(&e2);
        let det = dot(&e1, &pvec);

        if relative_eq!(det, 0.0) {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = ray.origin - self.p1;
        let u = dot(&tvec, &pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(&e1);
        let v = dot(ray.dir.as_ref(), &qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = dot(&e2, &qvec) * inv_det;

        if t < 0.0 {
            return None;
        }

        let mut n = normalize(&e2.cross(&e1));
        if dot(&n, ray.dir.as_ref()) > 0.0 {
            n = -n;
        }
        Some(PrimitiveIntersection {p: ray.origin + t*ray.dir.unwrap(), n: n, tang: e1, d: t, u: u, v: v})
    }

    // fn intersect_prim(&self, ray: &Ray) -> Option<PrimitiveIntersection> {
    //     let p = self.p1;
    //     let u = self.p2 - self.p1;
    //     let v = self.p3 - self.p1;

    //     let n = normalize(&u.cross(&v));
    //     let b = dot(&n, ray.dir.as_ref());

    //     let to_ray_origin = ray.origin - p;
    //     // distance to the plane that the triangle lies on
    //     let dist = dot(&n, &to_ray_origin) / -b;

    //     if relative_eq!(b, 0.0) || dist < f32::EPSILON {
    //         return None;
    //     }

    //     // Intersection of the ray and the plane the triangle lies on (relative to the triangle center)
    //     let w = to_ray_origin + ray.dir.unwrap() * dist;

    //     let uu = dot(&u, &u);
    //     let vv = dot(&v, &v);
    //     let uv = dot(&u, &v);
    //     let wv = dot(&w, &v);
    //     let wu = dot(&w, &u);

    //     let inv_den = 1.0 / (uv*uv - uu*vv);
    //     // Barycentric coordinates of w
    //     let s = (uv*wv - vv*wu) * inv_den;
    //     let t = (uv*wu - uu*wv) * inv_den;

    //     if s < 0.0 || t < 0.0 || (s+t) > 1.0 {
    //         return None;
    //     }

    //     Some(PrimitiveIntersection {p: Point3::from_coordinates(w), n: n, d: dist, u: s, v: t})
    // }
}