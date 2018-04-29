

#[derive(Debug, Copy, Clone)]
pub struct RenderSettings {
    pub spp: u32,
    pub max_depth: u32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings {
            spp: 16,
            max_depth: 4,
        }
    }
}