extern crate image;

mod renderer;

fn main() {
    let renderer = renderer::build_renderer(480, 320);
    let path = &std::path::Path::new("render.png");
    let _ = image::save_buffer(path, renderer.img.as_slice(), renderer.w, renderer.h, image::RGB(8));
}
