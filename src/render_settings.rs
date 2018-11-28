

#[derive(Debug, Copy, Clone)]
pub struct RenderSettings {
    pub spp: usize,
    pub max_depth: u32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings {
            spp: 8,
            max_depth: 4,
        }
    }
}