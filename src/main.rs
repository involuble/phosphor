#[macro_use]
extern crate log;
extern crate fern;
extern crate image;
extern crate cgmath;
#[macro_use]
extern crate approx;
extern crate num_traits;
extern crate rand;

mod geometry;
mod scene;
mod renderer;
mod colour;
mod material;
mod camera;
mod lights;
mod sampling;
mod surface;
mod mesh;
mod linalg;

use cgmath::*;

use geometry::*;
use scene::*;
use colour::*;
use material::*;
use camera::*;
use surface::*;
use renderer::*;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        // .chain(fern::log_file("render_log.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    let _ = setup_logger();

    let mut scene = Scene::new();
    let camera = Camera::new(Point3::new(0.0, 3.0, -3.1), Vector3::unit_z(), Vector3::unit_y(), 90.0);

    let back_wall = vec![
        Triangle::new( Point3::new(-3.0, 0.0, 6.0), Point3::new(3.0, 0.0, 6.0), Point3::new(3.0, 6.0, 6.0)),
        Triangle::new( Point3::new(-3.0, 0.0, 6.0), Point3::new(3.0, 6.0, 6.0), Point3::new(-3.0, 6.0, 6.0)),
    ];
    scene.add_mesh(back_wall, Material::new(Colour::from_luma(0.9)));

    let bottom_wall = vec![
        Triangle::new( Point3::new(-3.0, 0.0, 0.0), Point3::new(3.0, 0.0, 0.0), Point3::new(3.0, 0.0, 6.0)),
        Triangle::new( Point3::new(-3.0, 0.0, 0.0), Point3::new(3.0, 0.0, 6.0), Point3::new(-3.0, 0.0, 6.0)),
    ];
    scene.add_mesh(bottom_wall, Material::new(Colour::from_luma(0.9)));

    let top_wall = vec![
        Triangle::new( Point3::new(-3.0, 6.0, 0.0), Point3::new(3.0, 6.0, 0.0), Point3::new(3.0, 6.0, 6.0)),
        Triangle::new( Point3::new(-3.0, 6.0, 0.0), Point3::new(3.0, 6.0, 6.0), Point3::new(-3.0, 6.0, 6.0)),
    ];
    scene.add_mesh(top_wall, Material::new(Colour::from_luma(0.9)));

    let right_wall = vec![
        Triangle::new( Point3::new(-3.0, 0.0, 0.0), Point3::new(-3.0, 0.0, 6.0), Point3::new(-3.0, 6.0, 6.0)),
        Triangle::new( Point3::new(-3.0, 0.0, 0.0), Point3::new(-3.0, 6.0, 6.0), Point3::new(-3.0, 6.0, 0.0)),
    ];
    scene.add_mesh(right_wall, Material::new(Colour::new(0.0, 1.0, 0.0)));

    let left_wall = vec![
        Triangle::new( Point3::new(3.0, 0.0, 0.0), Point3::new(3.0, 6.0, 6.0), Point3::new(3.0, 0.0, 6.0)),
        Triangle::new( Point3::new(3.0, 0.0, 0.0), Point3::new(3.0, 6.0, 0.0), Point3::new(3.0, 6.0, 6.0)),
    ];
    scene.add_mesh(left_wall, Material::new(Colour::new(1.0, 0.0, 0.0)));

    scene.add_sphere(SphereSurface::new(Point3::new(-1.5, 1.0, 3.0), 0.9, Material::new(Colour::from_luma(1.0))));
    scene.add_sphere(SphereSurface::new(Point3::new(1.5, 1.0, 4.0), 0.9, Material::new(Colour::from_luma(1.0))));

    scene.add_light(SphereSurface::new(Point3::new(0.0, 5.3, 2.0), 0.5, Material::new_emitter(Colour::from_luma(1.0))));

    let mut furnace_scene = Scene::new();
    let furnace_c = Colour::from_luma(0.18); // = 118 after sRGB encoding
    furnace_scene.add_sphere(SphereSurface::new(Point3::new(0.0, 3.0, 1.0), 2.0, Material::new(furnace_c)));
    // println!("Linear colour {} to sRGB = {}", furnace_c.r, furnace_c.into_u8_rgb()[0]); // 0.18 & 118
    furnace_scene.background = Colour::from_luma(1.0);

    let mut renderer = Renderer::build_renderer(scene, camera, 320, 240);
    renderer.render();

    let path = std::path::Path::new("render.png");
    let img = renderer.get_srgb_img_buf();
    let saved = image::save_buffer(
        path,
        img.as_ref(),
        renderer.w,
        renderer.h,
        image::RGB(8),
    );
    match saved {
        Ok(_) => info!("Image written successfully"),
        Err(e) => error!("Image couldn't be written: {}", e),
    }
}
