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
            let _obj = load_obj(path, true)?;
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
        "obj" => {
            dbg!(path);
            let (models, _) = tobj::load_obj(path, true)?;
            let mut data = TriangleMesh::default();
            for m in models {
                let base = data.verts.len() as u32;
                for idx in m.mesh.indices.chunks(3) {
                    let tri = [idx[0] + base, idx[1] + base, idx[2] + base, 0];
                    data.tris.push(tri);
                }
                let vertices = m.mesh.positions.len() / 3;
                for i in 0..vertices {
                    let vertex = Vertex {
                        pos: [m.mesh.positions[i*3], m.mesh.positions[i*3 + 1], m.mesh.positions[i*3 + 2]],
                        normal: [m.mesh.normals[i*3], m.mesh.normals[i*3 + 1], m.mesh.normals[i*3 + 2]],
                        uv: [m.mesh.texcoords[i*2], m.mesh.texcoords[i*2 + 1]],
                    };
                    data.verts.push(vertex);
                }
            }
            data
        }
        format => return Err(format!("Unknown mesh file format {}", format).into()),
    })
}
