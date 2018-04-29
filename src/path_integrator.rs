use rand::{Rng, thread_rng};

use math::*;
use geometry::*;
use scene::*;
use colour::*;
use camera::*;
use render_buffer::*;
use render_settings::*;
use materials::*;

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

        render_buffer.data.iter_mut().enumerate().for_each(|(index, pixel)| {
            let x = index % (width as usize);
            let y = index / (width as usize);
            // if !(x == 160 && y == 70) { return; }
            let x = (x as f32) * inv_w;
            let y = (y as f32) * inv_h;

            let mut rng = thread_rng();
            for _ in 0..self.spp {
                let r1 = rng.next_f32();
                let r2 = rng.next_f32();
                let offset_x = r1 * inv_w;
                let offset_y = r2 * inv_h;

                let camera_ray = camera.get_ray(x + offset_x, y + offset_y);
                let radiance = self.radiance(&camera_ray, &mut rng);
                pixel.add_sample(radiance);
            }
        })
    }

    pub fn radiance<R: Rng>(&self, camera_ray: &Ray, rng: &mut R) -> Colour {
        let mut hit = self.scene.intersect(camera_ray);

        if !hit.is_hit() {
            return self.scene.eval_skybox(camera_ray.dir);
        }

        let light_sample = self.scene.emission_at(&camera_ray, &hit);

        if !light_sample.radiance.is_zero() {
            return light_sample.radiance;
        }

        let first_material = self.scene.get_material(hit.geom_id);
        return first_material.albedo();

        let mut radiance = Colour::zero();
        let mut reflectance = Colour::new(1.0, 1.0, 1.0);

        for _ in 0..self.max_depth {
            //
        }
        radiance
    }
}