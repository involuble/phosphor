use crate::math::*;
use embree::{Ray, UserPrimHit, UserPrimitive, Bounds};
use crate::colour::*;
use crate::geometry::{SampleableEmitter, LightSample};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub emission: Colour,
}

impl Sphere {
    pub fn unit() -> Self {
        Sphere {
            center: Vec3::zero(),
            radius: 1.0,
            emission: Colour::zero(),
        }
    }

    pub fn is_emissive(&self) -> bool {
        !self.emission.is_zero()
    }
}

fn sample_cone(xi: [f32; 2], cos_theta_max: f32) -> Vec3 {
    let u1: f32 = xi[0];
    let u2: f32 = xi[1];

    let cos_theta = (1.0 - u1) + u1*cos_theta_max;
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    let phi = 2.0 * PI * u2;

    let v = polar_to_cartesian(sin_theta, cos_theta, phi);
    v
}

impl SampleableEmitter for Sphere {
    fn eval_emission_at(&self, initial: Vec3, p: Vec3) -> LightSample {
        let distance = self.center.distance_squared(initial);
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

    fn sample(&self, xi: [f32; 2], initial: Vec3) -> LightSample {
        // See https://www.akalin.com/sampling-visible-sphere
        //  if a point on the sphere (rather than a direction) is needed
        let sin_theta_max2 = self.radius * self.radius / self.center.distance_squared(initial);
        if sin_theta_max2 >= 1.0 {
            // Sample by uniform area
            unimplemented!();
        }
        let cos_theta_max = (1.0 - sin_theta_max2).sqrt();

        let v = sample_cone(xi, cos_theta_max);

        let dist = self.center.distance(initial);
        let to = (self.center - initial) / dist;
        let (cone_x, cone_y) = make_orthonormal_basis(to);

        let d = v.x * cone_x + v.y * cone_y + v.z * to;
        debug_assert!((d.length() - 1.0).abs() < EPSILON);

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
            log::warn!("Can't transform sphere by non-uniform scale");
        }
        self.center = transform.transform_point(self.center);
        self.radius *= transform.scale.x;
    }
}

/// Computes a*d - b*c using a numerically stable method from
///     Kahan, "On the cost of floating-point computation without extra-precise arithmetic"
fn determinant(a: f32, b: f32, c: f32, d: f32) -> f32 {
    let w = b * c;
    let e = f32::mul_add(-b, c, w);
    let f = f32::mul_add(a, d, -w);
    f + e
}

/// Computes a*b - c*d using a numerically stable method from
///     Kahan, "On the cost of floating-point computation without extra-precise arithmetic"
///     <https://hal.inria.fr/ensl-00649347/en>
fn diff_of_products(a: f32, b: f32, c: f32, d: f32) -> f32 {
    let w = c * d;
    let e = f32::mul_add(c, d, -w);
    let f = f32::mul_add(a, b, -w);
    f - e
}

impl UserPrimitive for Sphere {
    fn intersect(&self, ray: &Ray) -> UserPrimHit {
        // Use a numerically stable algorithm from https://en.wikipedia.org/wiki/Loss_of_significance#A_better_algorithm
        let v = ray.origin - self.center;

        let a = ray.dir.length_squared();
        let b = 2.0 * dot(v, ray.dir);
        let c = v.length_squared() - self.radius * self.radius;

        let d = determinant(b, 4.0 * a, c, b);

        if d < 0.0 {
            return UserPrimHit::miss()
        }

        let q = -0.5 * (b + b.signum() * d.sqrt());

        let t0 = c / q;
        if ray.in_range(t0) {
            return UserPrimHit {
                t: t0,
                Ng: ray.point_at_dist(t0) - self.center,
                uv: Vec2::zero(),
            }
        }
        
        let t1 = q / a;
        if ray.in_range(t1) {
            return UserPrimHit {
                t: t1,
                Ng: ray.point_at_dist(t1) - self.center,
                uv: Vec2::zero(),
            }
        }
        UserPrimHit::miss()
    }

    fn bounds(&self) -> Bounds {
        Bounds::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius))
    }
}

#[derive(Debug, Clone)]
pub struct InfiniteSphereCap {
    pub cap_dir: Vec3,
    pub cap_angle: f32,
    pub emission: Colour,
}

impl SampleableEmitter for InfiniteSphereCap {
    fn eval_emission_at(&self, _initial: Vec3, _p: Vec3) -> LightSample {
        unimplemented!()
    }

    fn sample(&self, xi: [f32; 2], _initial: Vec3) -> LightSample {
        let cos_theta_max = self.cap_angle.cos();

        let v = sample_cone(xi, cos_theta_max);

        let (cone_x, cone_y) = make_orthonormal_basis(self.cap_dir);

        let d = v.x * cone_x + v.y * cone_y + v.z * self.cap_dir;

        LightSample {
            dir: d.normalize(),
            distance: f32::INFINITY,
            radiance: self.emission,
            pdf: PdfW(1.0 / (2.0 * PI * (1.0 - cos_theta_max))),
        }
    }

    fn surface_area(&self) -> f32 {
        f32::INFINITY
    }
}