mod scene;

pub use scene::*;

use std::error::Error;
use std::fs::read_to_string;
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
            serde_json::from_str(&read_to_string(path)?)?
        },
        "obj" => {
            let _obj = load_obj(path)?;
            todo!()
        },
        format => return Err(format!("Unknown file extension {}", format).into()),
    })
}
