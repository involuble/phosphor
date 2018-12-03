use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct SceneDescription {
    pub media: Vec<Medium>,
    pub bsdfs: Vec<BSDFEntry>,
    pub primitives: Vec<Primitive>,
    pub camera: Camera,
    pub integrator: IntegratorSettings,
    pub renderer: RendererSettings,
}

#[derive(Deserialize)]
pub struct BSDFEntry {
    pub name: String,

    #[serde(flatten)]
    pub bsdf: BSDF,

    #[serde(deserialize_with = "vector_or_scalar")]
    pub albedo: [f32; 3],
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum BSDF {
    Null,
    Lambert {},
}

fn float3_zero() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

#[derive(Deserialize)]
#[serde(untagged)]
enum VectorOrScalar {
    Scalar(f32),
    Vector([f32; 3]),
}

pub fn vector_or_scalar<'de, D>(deserializer: D) -> Result<[f32; 3], D::Error>
    where D: Deserializer<'de> {
    match VectorOrScalar::deserialize(deserializer)? {
        VectorOrScalar::Vector(v) => Ok(v),
        VectorOrScalar::Scalar(s) => Ok([s, s, s]),
    }
}

pub fn option_vector_or_scalar<'de, D>(deserializer: D) -> Result<Option<[f32; 3]>, D::Error>
    where D: Deserializer<'de> {
    vector_or_scalar(deserializer).map(|v| Some(v))
}

#[derive(Deserialize)]
pub struct Primitive {
    #[serde(flatten)]
    pub primitive: PrimitiveType,

    pub transform: Transform,
    pub bsdf: String,
    #[serde(default)]
    #[serde(deserialize_with = "option_vector_or_scalar")]
    pub emission: Option<[f32; 3]>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum PrimitiveType {
    Quad,
    Sphere,
    Cube,
}

#[derive(Deserialize)]
pub struct Transform {
    #[serde(default = "float3_zero")]
    pub position: [f32; 3],
    #[serde(deserialize_with = "vector_or_scalar")]
    pub scale: [f32; 3],
    /// Euler angles specified in degrees
    pub rotation: [f32; 3],
}

#[derive(Deserialize)]
pub struct Camera {
    pub tonemap: String,
    pub resolution: [u32; 2],
    pub reconstruction_filter: String,
    pub transform: CameraTransform,
    #[serde(rename = "fov")]
    pub fov_degrees: f32,
    #[serde(rename = "type")]
    pub camera_type: String,
}

#[derive(Deserialize)]
pub struct CameraTransform {
    pub position: [f32; 3],
    pub look_at: [f32; 3],
    pub up: [f32; 3],
}

#[derive(Deserialize)]
pub struct IntegratorSettings {
    #[serde(rename = "type")]
    pub integrator_type: String,
    pub min_bounces: u32,
    pub max_bounces: u32,
    pub enable_consistency_checks: bool,
    pub enable_two_sided_shading: bool,
    #[serde(default)]
    pub enable_light_sampling: bool,
    #[serde(default)]
    pub enable_volume_light_sampling: bool,
}

#[derive(Deserialize)]
pub struct RendererSettings {
    pub output_file: String,
    pub resume_render_file: String,
    pub overwrite_output_files: bool,
    pub adaptive_sampling: bool,
    pub enable_resume_render: bool,
    pub stratified_sampler: bool,
    pub scene_bvh: bool,
    pub spp: u32,
    pub spp_step: u32,
    pub checkpoint_interval: String,
    pub timeout: String,
    pub hdr_output_file: String,
}

#[derive(Deserialize)]
pub struct Medium {
}