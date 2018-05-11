use rand::{Rng, IsaacRng, SeedableRng};
use embree::{Hit};
use rayon::prelude::*;

use math::*;
use geometry::*;
use scene::*;
use colour::*;
use camera::*;
use render_buffer::*;
use render_settings::*;

pub struct PathIntegrator {
    scene: Scene,
    spp: u32,
    max_depth: u32,
}

impl PathIntegrator {
    pub fn new(scene: Scene, settings: &RenderSettings) -> Self {
        PathIntegrator {
            scene: scene,
            spp: settings.spp,
            max_depth: settings.max_depth,
        }
    }

    pub fn render(&self, camera: &Camera, render_buffer: &mut RenderBuffer) {
        let width = render_buffer.width;
        let inv_w = 1.0 / (render_buffer.width as f32);
        let inv_h = 1.0 / (render_buffer.height as f32);

        render_buffer.data.par_iter_mut().enumerate().for_each(|(index, pixel)| {
            let x_i = (index as u32) % width;
            let y_i = (index as u32) / width;
            // if !(x_i == 505 && y_i == 90) { return; }
            let x = (x_i as f32) * inv_w;
            let y = (y_i as f32) * inv_h;

            for sample_i in 0..self.spp {
                let mut rng = IsaacRng::from_seed([31415, x_i, y_i, sample_i].as_ref());
                let r1 = rng.next_f32();
                let r2 = rng.next_f32();
                let offset_x = r1 * inv_w;
                let offset_y = r2 * inv_h;

                let camera_ray = camera.get_ray(x + offset_x, y + offset_y);
                let mut radiance = self.radiance(&camera_ray, &mut rng);
                if radiance.is_nan() {
                    warn!("NaN colour at pixel ({},{})", x_i, y_i);
                    radiance = Colour::new(1.0, 0.4, 0.7); // A vibrant pink colour
                }
                pixel.add_sample(radiance);
            }
        })
    }

    pub fn radiance(&self, camera_ray: &Ray, rng: &mut IsaacRng) -> Colour {
        let mut ray = *camera_ray;

        let mut radiance = Colour::zero();
        let mut reflectance = Colour::new(1.0, 1.0, 1.0);

        for depth in 0..self.max_depth {
            let mut hit = self.scene.intersect(&ray);

            if same_hemisphere(hit.Ng, ray.dir) {
                hit.Ng = -hit.Ng;
            }

            if !hit.is_hit() {
                radiance += reflectance * self.scene.skybox_emission(camera_ray.dir);
                break;
            }

            let light_sample = self.scene.emission_at(&camera_ray, &hit);
            let weight = if depth == 0 { 1.0 } else { 0.0 };
            radiance += reflectance * weight * light_sample.radiance;

            let shading = self.scene.shading_parameters_at(&hit);

            radiance += reflectance * self.direct_light_sample(rng, &ray, &hit, &shading);

            let bsdf_sample = shading.bsdf.sample(rng, &shading.basis, ray.dir);

            if bsdf_sample.pdf.0 > EPSILON {
                reflectance *= bsdf_sample.reflectance * dot(bsdf_sample.w_o, hit.Ng) / bsdf_sample.pdf.0;
            } else {
                reflectance = Colour::zero();
            }
            debug_assert!(reflectance.r >= 0.0 && reflectance.r <= 1.0, "Reflectance should be in [0,1]");
            debug_assert!(reflectance.g >= 0.0 && reflectance.g <= 1.0, "Reflectance should be in [0,1]");
            debug_assert!(reflectance.b >= 0.0 && reflectance.b <= 1.0, "Reflectance should be in [0,1]");

            ray = Ray::new(ray.point_at_dist(hit.t), bsdf_sample.w_o, ::std::f32::MAX);
            ray.offset(hit.Ng);
        }
        radiance
    }

    fn direct_light_sample(&self, rng: &mut IsaacRng, ray: &Ray, hit: &Hit, shading: &ShadingParameters) -> Colour {
        if self.scene.lights.len() == 0 {
            return Colour::zero();
        }

        // https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
        fn rand_select<'a, T>(vec: &'a Vec<T>, rand: u32) -> &'a T {
            let i = ((vec.len() as u64) * (rand as u64)) >> 32;
            &vec[i as usize]
        };

        let (light_id, light) = rand_select(&self.scene.lights, rng.next_u32());
        let light_id = *light_id;

        if hit.geom_id == light_id {
            return Colour::zero();
        }

        let hit_p = ray.point_at_dist(hit.t);
        let light_sample = light.sample(rng, hit_p);

        let n_dot_l = dot(shading.basis.normal, light_sample.dir);
        if n_dot_l > EPSILON && light_sample.pdf.0 > EPSILON {
            let mut light_ray = Ray::new(hit_p, light_sample.dir, light_sample.distance);
            light_ray.offset(hit.Ng);
            let light_hit = self.scene.intersect(&light_ray);
            if light_id == light_hit.geom_id {
                let bsdf_sample = shading.bsdf.eval(&shading.basis, ray.dir, light_sample.dir);
                return light_sample.radiance * bsdf_sample.reflectance * n_dot_l / light_sample.pdf.0;
            }
        }
        Colour::zero()
    }
}