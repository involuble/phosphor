#[macro_use]
extern crate log;
extern crate env_logger;
extern crate image;
extern crate nalgebra as na;

mod primitive;
mod scene;
mod renderer;

use std::ops::Deref;

fn main() {
    let _ = env_logger::init();

    let scene = scene::Scene::new();
    let renderer = renderer::Renderer::build_renderer(scene, 480, 320);

    let path = &std::path::Path::new("render.png");
    let saved = image::save_buffer(path, renderer.img.deref(), renderer.w, renderer.h, image::RGB(8));
    match saved {
        Ok(_) => info!("Image written successfully"),
        Err(e) => error!("Image couldn't be written: {}", e),
    }
}
