#[macro_use]
extern crate log;
extern crate env_logger;
extern crate image;
extern crate nalgebra as na;
#[macro_use]
extern crate approx;

mod primitive;
mod scene;
mod renderer;
mod colour;

use std::f32::consts::PI;
use std::ops::Deref;
use na::*;

use primitive::*;
use scene::*;
use colour::*;

fn main() {
    let _ = env_logger::init();

    let mut scene = Scene::new();
    scene.camera = Camera { loc: Point3::new(0.0, 3.0, -4.0), forward: Vector3::z(), up: Vector3::y(), fov: PI/2.0 };

    scene.spheres.push(Sphere::new(Point3::new(-1.5, 1.0, 4.0), 0.9));
    scene.tris.push(Triangle::new(Point3::new(-3.0, 0.0, 6.0), Point3::new( 3.0, 0.0, 6.0), Point3::new( 3.0, 6.0, 6.0)));
    scene.tris.push(Triangle::new(Point3::new(-3.0, 0.0, 6.0), Point3::new( 3.0, 6.0, 6.0), Point3::new(-3.0, 6.0, 6.0)));
    let mut renderer = renderer::Renderer::build_renderer(scene, 320, 240);

    renderer.render();

    let path = std::path::Path::new("render.png");
    let saved = image::save_buffer(path, renderer.img.deref(), renderer.w, renderer.h, image::RGB(8));
    match saved {
        Ok(_) => info!("Image written successfully"),
        Err(e) => error!("Image couldn't be written: {}", e),
    }
}
