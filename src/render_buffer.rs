pub use colour::*;
pub use tonemap::*;

#[derive(Debug, Clone)]
pub struct RenderBuffer {
    pub data: Vec<PixelRecord>,
    pub width: u32,
    pub height: u32,
}

impl RenderBuffer {
    pub fn new(w: u32, h: u32) -> Self {
        let len = w * h;
        RenderBuffer {
            data: vec![PixelRecord::empty(); len as usize],
            width: w,
            height: h,
        }
    }

    pub fn resolve(&self) -> Image {
        let mut v = Vec::with_capacity((self.width * self.height) as usize);
        for pixel in &self.data {
            // debug_assert!(pixel.sample_count > 0.0);
            let mut colour = Colour::zero();
            if pixel.sample_count > 0.0 {
                colour += pixel.colour_sum / pixel.sample_count;
            }
            v.push(colour);
        }
        Image {
            image: v,
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PixelRecord {
    pub colour_sum: Colour,
    pub sample_count: f32,
}

impl PixelRecord {
    pub fn empty() -> Self {
        PixelRecord {
            colour_sum: Colour::zero(),
            sample_count: 0.0,
        }
    }

    pub fn add_sample(&mut self, c: Colour) {
        self.colour_sum += c;
        self.sample_count += 1.0;
    }
}
