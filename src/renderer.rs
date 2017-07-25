pub struct Renderer {
    pub w: u32,
    pub h: u32,
    pub img: Vec<u8>,
}

pub fn build_renderer(width: u32, height: u32) -> Renderer {
    let len = width*height;
    let buf = Vec::with_capacity(len as usize);
    Renderer { w: width, h: height, img: buf }
}
