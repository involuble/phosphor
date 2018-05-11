use rand::{Rng, IsaacRng};

use math::*;
use colour::*;
use geometry::{SampleableEmitter, LightSample};

#[derive(Debug, Clone)]
pub struct Quad {
    p0: Point3<f32>,
    p1: Point3<f32>,
    p2: Point3<f32>,
    p3: Point3<f32>,
    normal: Vector3<f32>,
    pub emission: Colour,
}

impl Quad {
    pub fn new(p0: Point3<f32>, p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) -> Quad {
        Quad {
            p0: p0,
            p1: p1,
            p2: p2,
            p3: p3,
            normal: (p1 - p0).cross(p3 - p0).normalize(),
            emission: Colour::zero(),
        }
    }

    pub fn is_emissive(&self) -> bool {
        !self.emission.is_zero()
    }

    pub fn points(&self) -> [Point3<f32>; 4] {
        [self.p0, self.p1, self.p2, self.p3]
    }
}

impl SampleableEmitter for Quad {
    fn eval_emission_at(&self, initial: Point3<f32>, p: Point3<f32>) -> LightSample {
        let pdf = PdfA(1.0 / self.surface_area());
        let dist = (p - initial).magnitude();
        let dir = (p - initial) / dist;
        let cos_theta = dot(self.normal, dir);
        LightSample {
            dir: dir,
            distance: dist,
            radiance: self.emission,
            pdf: pdf.to_pdfw(dist * dist, cos_theta),
        }
    }

    fn sample(&self, rng: &mut IsaacRng, initial: Point3<f32>) -> LightSample {
        let u1 = rng.next_f32();
        let u2 = rng.next_f32();

        let edge1 = self.p1 - self.p0;
        let edge2 = self.p3 - self.p0;

        let p = self.p0 + edge1 * u1 + edge2 * u2;

        self.eval_emission_at(initial, p)
    }

    fn surface_area(&self) -> f32 {
        (self.p1 - self.p0).cross(self.p3 - self.p0).magnitude()
    }
}

impl Transformable for Quad {
    fn transform_by(&mut self, transform: &AffineTransform) {
        self.p0 = transform.transform_point(self.p0);
        self.p1 = transform.transform_point(self.p1);
        self.p2 = transform.transform_point(self.p2);
        self.p3 = transform.transform_point(self.p3);
        self.normal = transform.transform_normal(self.normal);
    }
}