use primitive::*;
use scene::*;

pub struct Renderer {
    pub scene: Scene,
    pub w: u32,
    pub h: u32,
    pub img: Box<[u8]>,
}

impl Renderer {
    pub fn build_renderer(scene: Scene, w: u32, h: u32) -> Renderer {
        let len = w * h * 3;
        let buf = vec![0; len as usize];
        Renderer {
            scene: scene,
            w: w,
            h: h,
            img: buf.into_boxed_slice(),
        }
    }
}
