#![allow(dead_code)]

mod math;
mod colour;
mod geometry;
mod materials;
mod sampling;

mod scene;
mod path_integrator;
mod camera;
mod render_buffer;
mod image_buffer;

use std::time::Instant;
use std::fs::File;
use std::io::BufWriter;
use std::error::Error;
use std::fmt;

use argh::FromArgs;

use scene_desc::load_scene;

use crate::scene::*;
use crate::path_integrator::*;
use crate::render_buffer::*;

/// Render the given scene file
#[derive(FromArgs)]
struct RenderCommand {
    /// samples per pixel
    #[argh(option, short = 's')]
    samples: Option<u32>,
    /// input scene file
    #[argh(positional)]
    scene_file: String,
}

struct Timer {
    start: Instant,
}

impl Timer {
    pub fn start() -> Self {
        Timer { start: Instant::now() }
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = Instant::now() - self.start;

        let minutes = d.as_secs() / 60;
        let sec = d.as_secs() % 60;
        let millis = d.subsec_millis();
        if minutes > 30 {
            let hours = minutes / 60;
            let minutes = minutes % 60;
            write!(f, "{:02}h{:02}m", hours, minutes)
        } else if minutes > 0 {
            write!(f, "{}m{}s", minutes, sec)
        } else {
            write!(f, "{}.{:03}s", sec, millis)
        }
    }
}

const DEFAULT_SPP: u32 = 8;
const DEFAULT_BOUNCES: u32 = 4;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>>{
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug) // Trace is default
        .chain(std::io::stdout())
        // .chain(fern::log_file("render_log.log").expect("Unable to open log file"))
        .apply()?;
        // .expect("Unable to initialize logger");
    
    let config: RenderCommand = argh::from_env();

    let build_timer = Timer::start();
    
    let device = embree::Device::new();
    embree::set_flush_to_zero_mode();

    let mut scene_builder = SceneBuilder::new(&device);

    let scene_desc = load_scene(config.scene_file)?;

    let camera = scene_desc.camera.clone().into();

    scene_builder.load_scene(&scene_desc);

    let (width, height) = scene_desc.resolution();
    let mut render_buffer = RenderBuffer::new(width, height);

    let spp = config.samples.unwrap_or(DEFAULT_SPP);
    let mut path_integrator = PathIntegrator::new(scene_builder.build(), spp, DEFAULT_BOUNCES);
    if cfg!(not(debug_assertions)) {
        path_integrator.spp = scene_desc.renderer.spp;
        path_integrator.max_depth = scene_desc.integrator.max_bounces;
    }
    log::info!("{:>16} took: {}", "Scene building", build_timer);
    let render_timer = Timer::start();
    path_integrator.render(&camera, &mut render_buffer);
    log::info!("{:>16} took: {}", "Rendering", render_timer);

    let image = render_buffer.resolve();

    let image_ldr = image.to_ldr();

    let path = std::path::Path::new("render.png");
    let file = File::create(path)?;
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image.width as u32, image.height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&image_ldr)?;

    log::info!("Image written successfully");

    Ok(())
}