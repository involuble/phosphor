// use image;
use na::*;
use std::f32;

use primitive::*;
use scene::*;
use colour::*;

pub struct Renderer {
    pub scene: Scene,
    pub w: u32,
    pub h: u32,
    pub img: Box<[u8]>,
}

impl Renderer {
    pub fn build_renderer(scene: Scene, w: u32, h: u32) -> Self {
        let len = w * h * 3;
        let buf = vec![0; len as usize];
        Renderer {
            scene: scene,
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
        let fov_scale = (self.scene.camera.fov / 2.0).tan();
        let ss_x = (x as f32 + 0.5) / self.w as f32;
        let ss_x = ss_x * 2. - 1.;
        let ss_x = aspect_ratio_x * fov_scale * ss_x;
        let ss_y = (y as f32 + 0.5) / self.h as f32;
        let ss_y = 1. - ss_y * 2.0;
        let ss_y = aspect_ratio_y * fov_scale * ss_y;
        (ss_x, ss_y)
    }

    pub fn render(&mut self) {
        let camera_x = self.scene.camera.forward.cross(&self.scene.camera.up);
        for x in 0..self.w {
            for y in 0..self.h {
                let (ss_x, ss_y) = self.screen_space_coord(x, y);

                let camera_ray = self.scene.camera.forward + ss_x*camera_x + ss_y*self.scene.camera.up;
                let ray = Ray { origin: self.scene.camera.loc, dir: Unit::new_normalize(camera_ray) };

                let mut hit = None;
                let get_dist = |o: &Option<Intersection>| o.map_or(f32::INFINITY, |i| i.d);
                for prim in &self.scene.tris {
                    let new_hit = prim.intersect(&ray);
                    if get_dist(&new_hit) < get_dist(&hit) {
                        hit = new_hit;
                    }
                }
                for prim in &self.scene.spheres {
                    let new_hit = prim.intersect(&ray);
                    if get_dist(&new_hit) < get_dist(&hit) {
                        hit = new_hit;
                    }
                }

                let c;
                if let Some(i) = hit {
                    let mat = self.scene.materials[i.material_id as usize];
                    c = mat.colour;
                } else {
                    c = Colour::black();
                }
                self.set_pixel(x, y, c);
            }
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        let r = (colour.r * 255.0) as u8;
        let g = (colour.g * 255.0) as u8;
        let b = (colour.b * 255.0) as u8;
        let index = y * self.w + x;
        let i = (index*3) as usize;
        self.img[i  ] = r;
        self.img[i+1] = g;
        self.img[i+2] = b;
    }
}
