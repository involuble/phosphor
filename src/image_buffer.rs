use colour::*;

pub struct Image {
    pub image: Vec<Colour>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn to_ldr(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.image.len() * 3);
        for p in &self.image {
            let c = sRgb::from(*p).to_rgb8();
            buf.extend_from_slice(&c);
        }
        buf
    }
}