#[macro_use]
extern crate log;
extern crate fern;
extern crate image;
extern crate cgmath;
extern crate approx;
extern crate num_traits;
extern crate rand;
extern crate rand_pcg;
extern crate tobj;
extern crate vec_map;
extern crate rayon;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;
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
mod render_settings;
mod tungsten_scene;
mod tungsten_scene_convert;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

use crate::scene::*;
use crate::path_integrator::*;
use crate::render_buffer::*;
use crate::render_settings::*;

const DEFAULT_SPP: u32 = 8;
const DEFAULT_BOUNCES: u32 = 4;

fn load_scene<P: AsRef<Path>>(path: P) -> Result<tungsten_scene::SceneDescription, Box<Error>> {
    let file = File::open(path)?;

    let s = serde_json::from_reader(file)?;
    Ok(s)
}

fn pretty_duration(d: ::std::time::Duration) -> String {
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

fn main() {
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug) // Trace is default
        .chain(std::io::stdout())
        // .chain(fern::log_file("render_log.log").expect("Unable to open log file"))
        .apply()
        .expect("Unable to initialize logger");

    let build_start = Instant::now();
    
    let device = embree::Device::new();

    let mut scene_builder = SceneBuilder::new(&device);

    let scene_desc = load_scene("scenes/cornell_box_spheres.json").expect("could not load scene");
    // let scene_desc = load_scene("scenes/tungsten/cornell-box/scene.json").expect("could not load scene");

    let camera = scene_desc.build_camera();

    scene_desc.add_primitives(&mut scene_builder);

    let (width, height) = scene_desc.resolution();
    let mut render_buffer = RenderBuffer::new(width, height);

    // let settings = scene_desc.render_settings();
    let settings = RenderSettings {
        spp: DEFAULT_SPP,
        max_depth: DEFAULT_BOUNCES,
        .. scene_desc.render_settings()
    };

    let path_integrator = PathIntegrator::new(scene_builder.build(), &settings);
    println!("{:>16} took: {}", "Scene building", pretty_duration(Instant::now() - build_start));
    let render_start = Instant::now();
    path_integrator.render(&camera, &mut render_buffer);
    println!("{:>16} took: {}", "Rendering", pretty_duration(Instant::now() - render_start));

    let image = render_buffer.resolve();

    let image_ldr = image.to_ldr();

    let path = std::path::Path::new("render.png");
    let saved = image::save_buffer(
        path,
        image_ldr.as_ref(),
        image.width as u32,
        image.height as u32,
        image::RGB(8),
    );
    match saved {
        Ok(_) => info!("Image written successfully"),
        Err(e) => error!("Image couldn't be written: {}", e),
    }
}