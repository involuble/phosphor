use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct SceneDescription {
    pub media: Vec<Medium>,
    pub bsdfs: Vec<Material>,
    pub primitives: Vec<Primitive>,
    pub camera: Camera,
    pub integrator: IntegratorSettings,
    pub renderer: RendererSettings,
}

impl SceneDescription {
    pub fn resolution(&self) -> (u32, u32) {
        (self.camera.resolution[0], self.camera.resolution[1])
    }
}

#[derive(Deserialize)]
pub struct Material {
    pub name: String,

    #[serde(flatten)]
    pub bsdf: MaterialType,

    #[serde(deserialize_with = "vector_or_scalar")]
    pub albedo: [f32; 3],
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum MaterialType {
    Null,
    Lambert {},
    RoughConductor {
        roughness: f32,
        material: String,
        distribution: String,
    },
}

fn float3_one() -> [f32; 3] {
    [1.0, 1.0, 1.0]
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

#[derive(Deserialize)]
pub struct Primitive {
    #[serde(flatten)]
    pub primitive: PrimitiveType,

    pub transform: Transform,
    #[serde(default)]
    pub bsdf: String,
    #[serde(default)]
    #[serde(deserialize_with = "vector_or_scalar")]
    pub emission: [f32; 3],
}

#[derive(Deserialize, Debug, Default, Copy, Clone)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

#[derive(Debug, Deserialize, Clone)]
pub struct TriangleMesh {
    pub verts: Vec<Vertex>,
    pub tris: Vec<[u32; 4]>,
}

impl Default for TriangleMesh {
    fn default() -> Self {
        TriangleMesh {
            verts: Vec::new(),
            tris: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum PrimitiveType {
    Quad,
    Sphere {
        #[serde(default)]
        power: f32,
    },
    Cube,
    Mesh {
        file: String,
        smooth: bool,
        backface_culling: bool,
        recompute_normals: bool,
        #[serde(skip_deserializing)]
        mesh_data: TriangleMesh,

    },
    InfiniteSphereCap {
        power: f32,
        sample: bool,
        cap_angle: f32,
    },
}

#[derive(Deserialize)]
pub struct Transform {
    #[serde(default)]
    pub position: [f32; 3],
    #[serde(default = "float3_one")]
    #[serde(deserialize_with = "vector_or_scalar")]
    pub scale: [f32; 3],
    /// Euler angles specified in degrees
    #[serde(default)]
    pub rotation: [f32; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            rotation: [0.0, 0.0, 0.0],
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Deserialize)]
pub struct Camera {
    pub tonemap: String,
    pub resolution: [u32; 2],
    pub reconstruction_filter: String,
    pub transform: CameraTransform,
    /// Field of view (in degrees)
    pub fov: f32,
    #[serde(rename = "type")]
    pub camera_type: String,
}

#[derive(Debug, Clone)]
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