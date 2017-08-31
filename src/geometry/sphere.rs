use na::*;

use intersection::*;

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

impl Intersectable for Sphere {
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