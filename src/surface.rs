use cgmath::*;
use rand;

use geometry::*;
use material::*;
use sampling::*;
use linalg::*;

#[derive(Debug, Clone, Copy)]
pub struct SurfaceInfo {
    pub n_shading: Vector3<f32>,
    pub material: Material,
}

pub trait Surface: Intersectable {
    fn get_surface_info(&self, i: &Intersection) -> SurfaceInfo;
}

#[derive(Debug, Clone, Copy)]
pub struct SphereSurface {
    pub center: Point3<f32>,
    pub radius: f32,
    pub material: Material,
    pub geom_id: u32,
}

impl SphereSurface {
    pub fn new(c: Point3<f32>, r: f32, mat: Material) -> Self {
        SphereSurface { center: c, radius: r, material: mat, geom_id: INVALID_GEOM_ID }
    }
}

impl SphereSurface {
    // Returns a point on the sphere
    // Reference: http://ompf2.com/viewtopic.php?f=3&t=1914 (and PBRT)
    // TODO
    // pub fn sample<R: rand::Rng>(&self, rng: &mut R, p: Point3<f32>) -> (Vector3<f32>, f32) {
    //     let sin_theta_max2 = self.radius * self.radius / distance_squared(&self.center, &p);
    //     assert!(sin_theta_max2 <= 1.0 && sin_theta_max2 >= 0.0);
    //     let cos_theta_max = (1.0 - sin_theta_max2).sqrt();
        
    //     let u1 = rng.next_f32();
    //     let u2 = rng.next_f32();

    //     let cos_theta = (1.0 - u1) + u1*cos_theta_max;
    //     let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    //     let phi = 2.0 * PI * u2;

    //     let to = (self.center - p).normalize();
    //     let (cone_x, cone_y) = orthonormal_basis(to);

    //     let d = v.x * cone_x + v.y * cone_y + v.z * to;
    //     (d, 1.0 / (2.0 * PI * (1.0 - cos_theta_max))
    // }

    pub fn sample_vec<R: rand::Rng>(&self, rng: &mut R, p: Point3<f32>) -> (Vector3<f32>, f32) {
        let sin_theta_max2 = self.radius * self.radius / self.center.distance2(p);
        assert!(sin_theta_max2 <= 1.0 && sin_theta_max2 >= 0.0);
        let cos_theta_max = (1.0 - sin_theta_max2).sqrt();
        let (v, cone_pdf) = UniformConeDistribution::sample(rng, cos_theta_max);

        let to = (self.center - p).normalize();
        let (cone_x, cone_y) = orthonormal_basis(to);

        let d = v.x * cone_x + v.y * cone_y + v.z * to;
        assert_relative_eq!(d.magnitude(), 1.0, epsilon=EPSILON);
        (d.normalize(), cone_pdf)
    }
}

impl Intersectable for SphereSurface {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let s = Sphere::new(self.center, self.radius);
        let mut o = s.intersect(ray);
        Intersection::set_geom_id(&mut o, self.geom_id);
        o
    }
}

impl Surface for SphereSurface {
    fn get_surface_info(&self, i: &Intersection) -> SurfaceInfo {
        SurfaceInfo { n_shading: i.n, material: self.material }
    }
}