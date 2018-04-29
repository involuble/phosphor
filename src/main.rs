#[macro_use]
extern crate log;
extern crate fern;
extern crate image;
extern crate cgmath;
#[macro_use]
extern crate approx;
extern crate num_traits;
extern crate rand;
extern crate tobj;
extern crate vec_map;
extern crate embree;

mod math;
mod colour;
mod geometry;
mod materials;

mod material;
mod scene;
mod path_integrator;
mod camera;
mod render_buffer;
mod tonemap;
mod render_settings;

use cgmath::*;

use geometry::*;
use scene::*;
use colour::*;
use material::*;
use materials::*;
use camera::*;
// use math::*;
use path_integrator::*;
use render_buffer::*;
use render_settings::*;

fn main() {
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug) // Trace is default
        .chain(std::io::stdout())
        // .chain(fern::log_file("render_log.log").expect("Unable to open log file"))
        .apply()
        .expect("Unable to initialize logger");
    
    let device = embree::Device::new();

    let mut scene_builder = SceneBuilder::new(&device);

    let s1 = Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        emission: Colour::zero(),
    };

    let s2 = Sphere {
        center: Point3::new(-3.0, 0.0, 0.0),
        radius: 0.8,
        emission: Colour::new(1.5, 1.5, 1.5),
    };

    scene_builder.add_sphere(s1, Material::Lambert(Lambert { albedo: Colour::new(1.0, 0.0, 0.0)}));

    scene_builder.add_emissive_sphere(s2);

    let camera_pos = Point3::new(3.0, 4.0, 6.0);
    let camera = Camera::new(camera_pos, Point3::new(0.0, 0.0, 0.0), Vector3::unit_y(), Deg(60.0), 320.0 / 240.0);

    let mut render_buffer = RenderBuffer::new(320, 240);

    let path_integrator = PathIntegrator::new(scene_builder.build(), &RenderSettings::default());
    path_integrator.render(&camera, &mut render_buffer);

    let image = render_buffer.resolve();

    let image_buf = image.to_ldr();

    let path = std::path::Path::new("render.png");
    let saved = image::save_buffer(
        path,
        image_buf.as_ref(),
        image.width,
        image.height,
        image::RGB(8),
    );
    match saved {
        Ok(_) => info!("Image written successfully"),
        Err(e) => error!("Image couldn't be written: {}", e),
    }
}