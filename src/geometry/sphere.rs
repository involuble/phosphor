use crate::math::*;
use embree::{Ray, UserPrimHit, UserPrimitive, AABB};
use crate::colour::*;
use crate::geometry::{SampleableEmitter, LightSample};
use crate::sampling::*;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub emission: Colour,
}

impl Sphere {
    pub fn unit() -> Self {
        Sphere {
            center: Point3::origin(),
            radius: 1.0,
            emission: Colour::zero(),
        }
    }

    pub fn is_emissive(&self) -> bool {
        !self.emission.is_zero()
    }
}

fn sample_cone<R: Rng>(rng: &mut R, cos_theta_max: f32) -> Vector3<f32> {
    let u1: f32 = rng.gen();
    let u2: f32 = rng.gen();

    let cos_theta = (1.0 - u1) + u1*cos_theta_max;
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    let phi = 2.0 * PI * u2;

    let v = polar_to_cartesian(sin_theta, cos_theta, phi);
    v
}

impl SampleableEmitter for Sphere {
    fn eval_emission_at(&self, initial: Point3<f32>, p: Point3<f32>) -> LightSample {
        let distance = self.center.distance2(initial);
        let sin_theta_max2 = self.radius * self.radius / (distance * distance);
        let cos_theta_max = (1.0 - sin_theta_max2).sqrt();
        let pdf = 1.0 / (2.0 * PI * (1.0 - cos_theta_max));
        LightSample {
            dir: (p - initial).normalize(),
            distance: distance,
            radiance: self.emission,
            pdf: PdfW(pdf),
        }
    }

    fn sample(&self, rng: &mut SampleRng, initial: Point3<f32>) -> LightSample {
        // See https://www.akalin.com/sampling-visible-sphere
        //  if a point on the sphere (rather than a direction) is needed
        let sin_theta_max2 = self.radius * self.radius / self.center.distance2(initial);
        if sin_theta_max2 >= 1.0 {
            // Sample by uniform area
            unimplemented!();
        }
        let cos_theta_max = (1.0 - sin_theta_max2).sqrt();

        let v = sample_cone(rng, cos_theta_max);

        let dist = self.center.distance(initial);
        let to = (self.center - initial) / dist;
        let (cone_x, cone_y) = make_orthonormal_basis(to);

        let d = v.x * cone_x + v.y * cone_y + v.z * to;
        debug_assert!(relative_eq!(d.magnitude(), 1.0, epsilon=EPSILON));

        LightSample {
            dir: d,
            distance: dist,
            radiance: self.emission,
            pdf: PdfW(1.0 / (2.0 * PI * (1.0 - cos_theta_max))),
        }
    }

    fn surface_area(&self) -> f32 {
        4.0 * PI * self.radius * self.radius
    }
}

impl Transformable for Sphere {
    fn transform_by(&mut self, transform: &AffineTransform) {
        // debug_assert!(transform.is_similarity(), "Can't transform sphere by non-uniform scale");
        if !transform.is_similarity() {
            warn!("Can't transform sphere by non-uniform scale");
        }
        self.center = transform.transform_point(self.center);
        self.radius *= transform.scale.x;
    }
}

impl UserPrimitive for Sphere {
    fn intersect(&self, ray: &Ray) -> UserPrimHit {
        let v = ray.origin - self.center;

        let a = ray.dir.magnitude2();
        let b = 2.0 * dot(v, ray.dir);
        let c = v.magnitude2() - self.radius * self.radius;
        let d = b*b - 4.0 * a * c;
        if d < 0.0 {
            return UserPrimHit::miss()
        }

        let q = d.sqrt();
        let rcp_a = 1.0 / a;

        let t0 = 0.5 * rcp_a * (-b - q);
        if ray.in_range(t0) {
            return UserPrimHit {
                t: t0,
                Ng: ray.point_at_dist(t0) - self.center,
                uv: Vector2::zero(),
            }
        }
        let t1 = 0.5 * rcp_a * (-b + q);
        if ray.in_range(t1) {
            return UserPrimHit {
                t: t1,
                Ng: ray.point_at_dist(t1) - self.center,
                uv: Vector2::zero(),
            }
        }
        UserPrimHit::miss()
    }

    fn bounds(&self) -> AABB {
        AABB::new(
            self.center - Vector3::new(self.radius, self.radius, self.radius),
            self.center + Vector3::new(self.radius, self.radius, self.radius))
    }
}