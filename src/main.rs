#[macro_use]
extern crate log;
extern crate env_logger;
extern crate image;
extern crate nalgebra as na;
#[macro_use]
extern crate approx;
extern crate num_traits;
extern crate rand;

mod primitives;
mod scene;
mod renderer;
mod colour;
mod materials;
mod triangle_list;
mod camera;
mod lights;
mod samplers;

use na::*;

use primitives::*;
use scene::*;
use colour::*;
use materials::*;
use triangle_list::*;
use camera::*;
use lights::*;

// NOTE: Putting this here for reference
// trait OrdDebug: Ord + Debug {}
// impl<T: Ord + Debug> OrdDebug for T {}

fn main() {
    let _ = env_logger::init();

    let mut scene = Scene::new();
    let camera = Camera::new(Point3::new(0.0, 3.0, -4.0), Vector3::z(), Vector3::y(), 90.0);

    scene.add_material(Material::new(Colour::from_luma(0.9)));
    scene.add_material(Material::new(Colour::new(1.0, 0.0, 0.0)));
    scene.add_material(Material::new(Colour::new(0.0, 1.0, 0.0)));
    scene.add_material(Material::new(Colour::from_luma(1.0)));
    scene.add_material(Material::new(Colour::from_luma(1.0)));
    scene.add_material(Material::new_emitter(Colour::from_luma(1.0)));

    let back_wall: Box<[Triangle]> = Box::new([
        Triangle::new(
            Point3::new(-3.0, 0.0, 6.0),
            Point3::new(3.0, 0.0, 6.0),
            Point3::new(3.0, 6.0, 6.0),
        ),
        Triangle::new(
            Point3::new(-3.0, 0.0, 6.0),
            Point3::new(3.0, 6.0, 6.0),
            Point3::new(-3.0, 6.0, 6.0),
        ),
    ]);
    scene.add_triangle_list(TriangleList::from_vec(back_wall.into_vec(), 0));

    let bottom_wall: Box<[Triangle]> = Box::new([
        Triangle::new(
            Point3::new(-3.0, 0.0, 0.0),
            Point3::new(3.0, 0.0, 0.0),
            Point3::new(3.0, 0.0, 6.0),
        ),
        Triangle::new(
            Point3::new(-3.0, 0.0, 0.0),
            Point3::new(3.0, 0.0, 6.0),
            Point3::new(-3.0, 0.0, 6.0),
        ),
    ]);
    scene.add_triangle_list(TriangleList::from_vec(bottom_wall.into_vec(), 0));

    let left_wall: Box<[Triangle]> = Box::new([
        Triangle::new(
            Point3::new(-3.0, 0.0, 0.0),
            Point3::new(-3.0, 0.0, 6.0),
            Point3::new(-3.0, 6.0, 6.0),
        ),
        Triangle::new(
            Point3::new(-3.0, 0.0, 0.0),
            Point3::new(-3.0, 6.0, 6.0),
            Point3::new(-3.0, 6.0, 0.0),
        ),
    ]);
    scene.add_triangle_list(TriangleList::from_vec(left_wall.into_vec(), 1));

    let right_wall: Box<[Triangle]> = Box::new([
        Triangle::new(
            Point3::new(3.0, 0.0, 0.0),
            Point3::new(3.0, 6.0, 6.0),
            Point3::new(3.0, 0.0, 6.0),
        ),
        Triangle::new(
            Point3::new(3.0, 0.0, 0.0),
            Point3::new(3.0, 6.0, 0.0),
            Point3::new(3.0, 6.0, 6.0),
        ),
    ]);
    scene.add_triangle_list(TriangleList::from_vec(right_wall.into_vec(), 2));

    scene.add_sphere(Sphere::new(Point3::new(-1.5, 1.0, 4.1), 0.9, 3));
    scene.add_sphere(Sphere::new(Point3::new(1.5, 1.0, 4.0), 0.9, 4));

    scene.add_light(Sphere::new(Point3::new(0.0, 5.5, 2.0), 0.4, 5));

    let mut renderer = renderer::Renderer::build_renderer(scene, camera, 320, 240);
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
