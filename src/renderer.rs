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
    pub img: Box<[Colour]>,
}

impl Renderer {
    pub fn build_renderer(scene: Scene, w: u32, h: u32) -> Self {
        let len = w * h;
        let buf = vec![Colour::black(); len as usize];
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
                for prim in &self.scene.spheres {
                    let new_hit = prim.intersect(&ray);
                    if Intersection::get_dist(&new_hit) < Intersection::get_dist(&hit) {
                        hit = new_hit;
                    }
                }
                for prim in &self.scene.tri_lists {
                    let new_hit = prim.intersect(&ray);
                    if Intersection::get_dist(&new_hit) < Intersection::get_dist(&hit) {
                        hit = new_hit;
                    }
                }

                let c;
                if let Some(i) = hit {
                    let mat = self.scene.get_material(i.material_id);
                    c = mat.colour;
                } else {
                    c = Colour::black();
                }
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
        debug_assert!(buf.len() == buf.capacity());
        buf.into_boxed_slice()
    }
}
