mod scene_definition;
mod tungsten_scene;

pub use crate::tungsten_scene::*;

use std::error::Error;
use std::fs::{read, read_to_string};
use std::path::Path;

use tobj::load_obj;

fn file_ext(path: &Path) -> &str {
    path.extension().and_then(|ext| ext.to_str()).unwrap_or("")
}

pub fn load_scene<P: AsRef<Path>>(path: P) -> Result<SceneDescription, Box<dyn Error + Send + Sync>> {
    load_scene_impl(path.as_ref())
}

fn load_scene_impl(path: &Path) -> Result<SceneDescription, Box<dyn Error + Send + Sync>> {
    Ok(match file_ext(path) {
        "json" => {
            let mut scene: SceneDescription = serde_json::from_str(&read_to_string(path)?)?;
            for prim in scene.primitives.iter_mut() {
                match &mut prim.primitive {
                    PrimitiveType::Mesh { file, mesh_data, .. } => {
                        let base_path = path.parent().unwrap();
                        let mesh_path = base_path.join(file);
                        let mesh = load_mesh(mesh_path.as_ref())?;
                        *mesh_data = mesh;
                    }
                    _ => {},
                };
            }
            scene
        },
        "obj" => {
            let _obj = load_obj(path)?;
            todo!()
        },
        format => return Err(format!("Unknown scene file format {}", format).into()),
    })
}

fn load_mesh(path: &Path) -> Result<TriangleMesh, Box<dyn Error + Send + Sync>> {
    Ok(match file_ext(path) {
        "wo3" => {
            // let mut file = File::open(path)?;
            // let mut mesh: TriangleMesh = Default::default();

            // let mut buf = [0; 8];
            // file.read_exact(&mut buf)?;
            // let len = u64::from_le_bytes(buf);
            // mesh.verts = vec![Default::default(); len as usize];
            // mesh
            bincode::deserialize(&read(path)?)?
        },
        format => return Err(format!("Unknown mesh file format {}", format).into()),
    })
}
