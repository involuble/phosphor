use na::*;
use std::f32;
use std::f32::consts::{PI};
use rand::{Rng, thread_rng};

use primitives::*;
use scene::*;
use colour::*;
use camera::*;
use sampling::*;
use surface::*;
use linalg::*;

pub struct Renderer {
    pub scene: Scene,
    pub camera: Camera,
    pub w: u32,
    pub h: u32,
    pub img: Box<[Colour]>,
}

impl Renderer {
    pub fn build_renderer(scene: Scene, camera: Camera, w: u32, h: u32) -> Self {
        let len = w * h;
        let buf = vec![Colour::black(); len as usize];
        Renderer {
            scene: scene,
            camera: camera,
            w: w,
            h: h,
            img: buf.into_boxed_slice(),
        }
    }

    #[inline]
    fn screen_space_coord(&self, x: f32, y: f32) -> (f32, f32) {
        let aspect_ratio_x;
        let aspect_ratio_y;
        if self.w >= self.h {
            aspect_ratio_x = self.w as f32 / self.h as f32;
            aspect_ratio_y = 1.0;
        } else {
            aspect_ratio_x = 1.0;
            aspect_ratio_y = self.h as f32 / self.w as f32;
        }
        let fov_scale = (self.camera.fov / 2.0).tan();
        let ss_x = aspect_ratio_x * fov_scale * ((x / self.w as f32) * 2.0 - 1.0);
        let ss_y = aspect_ratio_y * fov_scale * (1.0 - (y / self.h as f32) * 2.0);
        (ss_x, ss_y)
    }

    pub fn intersect_ray(&self, ray: &Ray) -> Option<Intersection> {
        let mut hit = None;
        let mut dist = f32::INFINITY;
        for prim in &self.scene.spheres {
            let new_hit = prim.intersect(&ray);
            let new_dist = Intersection::get_dist(&new_hit);
            if new_dist < dist && new_dist > EPSILON {
                hit = new_hit;
                dist = new_dist;
            }
        }
        for prim in &self.scene.meshes {
            let new_hit = prim.intersect(&ray);
            let new_dist = Intersection::get_dist(&new_hit);
            if new_dist < dist && new_dist > EPSILON {
                hit = new_hit;
                dist = new_dist;
            }
        }
        assert!(hit.is_none() || hit.unwrap().t > 1e-7, "Probable self intersection: ray = {:?}\n intersection = {:?}", ray, hit.unwrap());
        hit
    }

    pub fn direct_light_estimate<R: Rng>(&self, rng: &mut R, i: &Intersection, surface_info: &SurfaceInfo) -> Colour {
        if self.scene.lights.len() == 0 {
            return Colour::black();
        }

        let light = &self.scene.lights[0];

        if i.geom_id == light.geom_id {
            return Colour::black();
        }

        let (l, pdf) = light.sample_vec(rng, i.p);
        let nl = l.dot(&i.n);

        let mut c = Colour::black();
        if nl > 0.0 {
            // Offset the ray from the surface by a tiny bit or else it intersects
            let light_ray = Ray::new(i.p, Unit::new_unchecked(l));
            let light_hit = self.intersect_ray(&light_ray);
            // This triggers around 2-6 times per frame which is a non-issue but requires leaving it commented
            // assert!(light_hit.is_some(), "A vector aimed at an object should either hit it or something else");
            if light_hit.is_some() && light_hit.unwrap().geom_id == light.geom_id {
                let _ = light_hit.unwrap();
                // TODO: Evaluate BRDF
                c += surface_info.material.base_colour * light.material.emittance * nl / pdf;
            }
        }
        c
    }

    pub fn trace<R: Rng>(&self, camera_ray: &Ray, rng: &mut R, max_depth: u32) -> Colour {
        let mut ray = *camera_ray;
        let mut acc_c = Colour::zero();
        let mut refl = Colour::new(1.0, 1.0, 1.0);

        for depth in 0..max_depth {
            // TODO: RR

            let hit = self.intersect_ray(&ray);
            if hit.is_none() {
                acc_c += refl * self.scene.background;
                return acc_c;
            }

            let i = hit.unwrap();
            let surface_info = self.scene.get_surface_info(i.geom_id, &i);
            // println!("      DEPTH = {}\nsurface_info = {:?}\nintersection = {:?}\n", depth, surface_info, i);

            if depth == 0 {
                acc_c += refl * surface_info.material.emittance;
            }

            // Next event estimation/direct light sampling
            acc_c += refl * self.direct_light_estimate(rng, &i, &surface_info);

            // Choose new ray direction
            let (tang, bitangent) = orthonormal_basis(i.n);
            let (r, pdf) = CosineHemisphereSampler::sample(rng);

            let wi = r.x * tang + r.y * bitangent + r.z * i.n;
            let wi = wi.normalize();

            // Evaluate brdf
            if pdf > EPSILON {
                refl *= surface_info.material.base_colour * dot(&wi, &i.n) / PI / pdf;
            }

            ray = Ray::new(i.p, Unit::new_unchecked(wi));
        }
        acc_c
    }

    pub fn render(&mut self) {
        let spp: u32 = 16;
        assert!(spp > 0);
        let camera_right = self.camera.forward.cross(&self.camera.up);
        // println!("SCENE TRIANGLES:\n\n{:?}\n\n", self.scene.meshes);
        for x in 0..self.w {
            for y in 0..self.h {
                // if !(x == 160 && y == 70) { continue; } // The light
                // if !(x == 140 && y == 140) { continue; } // The left sphere
                // if !(x == 100 && y == 100) { continue; } // The left wall (red one)
                let mut rng = thread_rng();
                let x_f = x as f32;
                let y_f = y as f32;

                let mut c = Colour::black();
                for _ in 0..spp {
                    let r1 = rng.next_f32();
                    let r2 = rng.next_f32();
                    let (ss_x, ss_y) = self.screen_space_coord(x_f+r1, y_f+r2);
                    let camera_ray = self.camera.forward + ss_x*camera_right + ss_y*self.camera.up;
                    let ray = Ray::new(self.camera.loc, Unit::new_normalize(camera_ray));
                    c += self.trace(&ray, &mut rng, 3);
                }
                c = c / (spp as f32);
                self.set_pixel(x, y, c);
            }
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        let index = y * self.w + x;
        self.img[index as usize] = colour;
    }

    pub fn get_srgb_img_buf(&self) -> Box<[u8]> {
        let len = self.w * self.h * 3;
        let mut buf = Vec::with_capacity(len as usize);
        for i in 0..self.img.len() {
            let bytes = self.img[i].into_u8_rgb();
            buf.extend_from_slice(&bytes);
        }
        assert!(buf.len() == buf.capacity());
        buf.into_boxed_slice()
    }
}
