use na::*;
use std::f32;

use primitives::*;
use scene::*;
use colour::*;
use camera::*;
use samplers::*;

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
    fn screen_space_coord(&self, x: u32, y: u32) -> (f32, f32) {
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
        let ss_x = (x as f32 + 0.5) / self.w as f32;
        let ss_x = ss_x * 2. - 1.;
        let ss_x = aspect_ratio_x * fov_scale * ss_x;
        let ss_y = (y as f32 + 0.5) / self.h as f32;
        let ss_y = 1. - ss_y * 2.0;
        let ss_y = aspect_ratio_y * fov_scale * ss_y;
        (ss_x, ss_y)
    }

    pub fn intersect_ray(&self, ray: &Ray) -> Option<SurfaceIntersection> {
        let mut hit = None;
        let dist = f32::INFINITY;
        for prim in &self.scene.spheres {
            let new_hit = prim.intersect(&ray);
            if SurfaceIntersection::get_dist(&new_hit) < dist {
                hit = new_hit;
            }
        }
        for prim in &self.scene.tri_lists {
            let new_hit = prim.intersect(&ray);
            if SurfaceIntersection::get_dist(&new_hit) < SurfaceIntersection::get_dist(&hit) {
                hit = new_hit;
            }
        }
        hit
    }

    pub fn trace(&self, camera_ray: &Ray, max_depth: u32) -> Colour {
        let mut ray = *camera_ray;
        let mut acc_c = Colour::zero();
        let mut refl = Colour::new(1.0, 1.0, 1.0);

        for i in 0..max_depth {
            let hit = self.intersect_ray(&ray);

            if hit.is_none() {
                return acc_c;
            }

            let i = hit.unwrap();
            let material = self.scene.get_material(i.material_id);

            acc_c += refl * material.emittance;
            refl *= material.base_colour;

            let light = &self.scene.lights[0];
            let to_light = light.sphere.center - i.prim_i.p;
            let d = to_light.norm();
            let l = to_light / d;
            let d = d - light.sphere.radius;
            // TODO: Do we ignore this light if dot(n,l) < 0?
            let nl = l.dot(&i.prim_i.n);

            if nl > 0.0 {
                // Offset the ray from the surface by a tiny bit or else it intersects
                const EPSILON: f32 = 1e-4;
                let shadow_ray = Ray::new(i.prim_i.p + l*EPSILON, Unit::new_unchecked(l));
                let shadow_hit = self.intersect_ray(&shadow_ray);
                if shadow_hit.is_none() || shadow_hit.unwrap().prim_i.d > d - 2.0*EPSILON {
                    acc_c += refl * light.emittance * nl;
                }
            }
        }
        acc_c
    }

    pub fn render(&mut self) {
        let camera_right = self.camera.forward.cross(&self.camera.up);
        for x in 0..self.w {
            for y in 0..self.h {
                // if !(x == 160 && y == 80) { continue; } // The light
                // if !(x == 140 && y == 140) { continue; } // The left sphere
                let (ss_x, ss_y) = self.screen_space_coord(x, y);

                let camera_ray = self.camera.forward + ss_x*camera_right + ss_y*self.camera.up;
                let ray = Ray::new(self.camera.loc, Unit::new_normalize(camera_ray));

                let c = self.trace(&ray, 1);
                self.set_pixel(x, y, c);
                // println!("{:?}", c);
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
        debug_assert!(buf.len() == buf.capacity());
        buf.into_boxed_slice()
    }
}
