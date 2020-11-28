use crate::math::*;
use crate::colour::*;
use crate::geometry::{SampleableEmitter, LightSample};

#[derive(Debug, Clone)]
pub struct Quad {
    p0: Vec3,
    edge1: Vec3,
    edge2: Vec3,
    normal: Vec3,
    pub emission: Colour,
}

impl Quad {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> Quad {
        let edge1 = p1 - p0;
        let edge2 = p3 - p0;
        let normal = edge1.cross(edge2).normalize();
        assert!((p0 + edge1 + edge2 - p2).length() < EPSILON, "Using edge representation for a quad causes accuracy problems");
        assert!(normal.length() > EPSILON, "Quad is degenerate");
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

    pub fn points(&self) -> [Vec3; 4] {
        [self.p0,
         self.p0 + self.edge1,
         self.p0 + self.edge1 + self.edge2,
         self.p0 + self.edge2]
    }
}

impl SampleableEmitter for Quad {
    fn eval_emission_at(&self, initial: Vec3, p: Vec3) -> LightSample {
        let pdf = PdfA(1.0 / self.surface_area());
        let dist = (p - initial).length();
        let dir = (p - initial) / dist;
        let cos_theta = dot(self.normal, dir).max(0.0);
        LightSample {
            dir: dir,
            distance: dist * 1.1,
            radiance: self.emission,
            pdf: pdf.to_pdfw(dist * dist, cos_theta),
        }
    }

    fn sample(&self, xi: [f32; 2], initial: Vec3) -> LightSample {
        let p = self.p0 + self.edge1 * xi[0] + self.edge2 * xi[1];

        self.eval_emission_at(initial, p)
    }

    fn surface_area(&self) -> f32 {
        self.normal.length()
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