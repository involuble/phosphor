#![allow(dead_code)]

#[macro_use]
extern crate log;
extern crate fern;
extern crate cgmath;
extern crate num_traits;
extern crate rand;
extern crate rand_pcg;
extern crate vec_map;
extern crate rayon;
extern crate embree;

mod math;
mod colour;
mod geometry;
mod materials;
mod sampling;

mod material_type;
mod scene;
mod path_integrator;
mod camera;
mod render_buffer;
mod image_buffer;

use std::time::Instant;
use std::fs::File;
use std::io::BufWriter;
use std::error::Error;
use std::time::Duration;

use scene_desc::load_scene;

use crate::scene::*;
use crate::path_integrator::*;
use crate::render_buffer::*;

fn pretty_duration(d: Duration) -> String {
    let minutes = d.as_secs() / 60;
    let sec = d.as_secs() % 60;
    let millis = d.subsec_millis();
    let pretty;
    if minutes > 30 {
        let hours = minutes / 60;
        let minutes = minutes % 60;
        pretty = format!("{:02}h{:02}m", hours, minutes);
    } else if minutes > 0 {
        pretty = format!("{}m{}s", minutes, sec);
    } else {
        pretty = format!("{}.{:03}s", sec, millis);
    }
    pretty
}

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>>{
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug) // Trace is default
        .chain(std::io::stdout())
        // .chain(fern::log_file("render_log.log").expect("Unable to open log file"))
        .apply()?;
        // .expect("Unable to initialize logger");

    let build_start = Instant::now();
    
    let device = embree::Device::new();
    embree::set_flush_to_zero_mode();

    let mut scene_builder = SceneBuilder::new(&device);

    let scene_desc = load_scene("scenes/cornell_box_spheres.json")?;
    // let scene_desc = load_scene("scenes/tungsten/cornell-box/scene.json")?;

    let camera = scene_desc.camera.clone().into();

    scene_builder.load_scene(&scene_desc);

    let (width, height) = scene_desc.resolution();
    let mut render_buffer = RenderBuffer::new(width, height);

    let mut path_integrator = PathIntegrator::new(scene_builder.build());
    if cfg!(not(debug_assertions)) {
        path_integrator.spp = scene_desc.renderer.spp;
        path_integrator.max_depth = scene_desc.integrator.max_bounces;
    }
    println!("{:>16} took: {}", "Scene building", pretty_duration(Instant::now() - build_start));
    let render_start = Instant::now();
    path_integrator.render(&camera, &mut render_buffer);
    println!("{:>16} took: {}", "Rendering", pretty_duration(Instant::now() - render_start));

    let image = render_buffer.resolve();

    let image_ldr = image.to_ldr();

    let path = std::path::Path::new("render.png");
    let file = File::create(path)?;
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image.width as u32, image.height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let saved = writer.write_image_data(&image_ldr);

    match saved {
        Ok(_) => info!("Image written successfully"),
        Err(e) => error!("Image couldn't be written: {}", e),
    };

    Ok(())
}