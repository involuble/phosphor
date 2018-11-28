use math::*;
use colour::*;
use geometry::{SampleableEmitter, LightSample};
use sampling::*;

#[derive(Debug, Clone)]
pub struct Quad {
    p0: Point3<f32>,
    edge1: Vector3<f32>,
    edge2: Vector3<f32>,
    normal: Vector3<f32>,
    pub emission: Colour,
}

impl Quad {
    pub fn new(p0: Point3<f32>, p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) -> Quad {
        let edge1 = p1 - p0;
        let edge2 = p3 - p0;
        let normal = edge1.cross(edge2).normalize();
        assert_relative_eq!(p0 + edge1 + edge2, p2, epsilon=EPSILON);
        assert!(normal.magnitude() > EPSILON, "Quad is degenerate");
        Quad {
            p0: p0,
            edge1: edge1,
            edge2: edge2,
            normal: normal,
            emission: Colour::zero(),
        }
    }

    pub fn is_emissive(&self) -> bool {
        !self.emission.is_zero()
    }

    pub fn points(&self) -> [Point3<f32>; 4] {
        [self.p0,
         self.p0 + self.edge1,
         self.p0 + self.edge1 + self.edge2,
         self.p0 + self.edge2]
    }
}

impl SampleableEmitter for Quad {
    fn eval_emission_at(&self, initial: Point3<f32>, p: Point3<f32>) -> LightSample {
        let pdf = PdfA(1.0 / self.surface_area());
        let dist = (p - initial).magnitude();
        let dir = (p - initial) / dist;
        let cos_theta = dot(self.normal, dir).max(0.0);
        LightSample {
            dir: dir,
            distance: dist * 1.1,
            radiance: self.emission,
            pdf: pdf.to_pdfw(dist * dist, cos_theta),
        }
    }

    fn sample(&self, rng: &mut SampleRng, initial: Point3<f32>) -> LightSample {
        let u1 = rng.gen();
        let u2 = rng.gen();

        let p = self.p0 + self.edge1 * u1 + self.edge2 * u2;

        self.eval_emission_at(initial, p)
    }

    fn surface_area(&self) -> f32 {
        self.normal.magnitude()
    }
}

impl Transformable for Quad {
    fn transform_by(&mut self, transform: &AffineTransform) {
        self.p0 = transform.transform_point(self.p0);
        self.edge1 = transform.transform_vector(self.edge1);
        self.edge2 = transform.transform_vector(self.edge2);
        self.normal = transform.transform_normal(self.normal);
    }
}