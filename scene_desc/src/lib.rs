mod scene;

pub use scene::*;

use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn load_scene<P: AsRef<Path>>(path: P) -> Result<SceneDescription, Box<dyn Error + Send + Sync>> {
    load_scene_impl(path.as_ref())
}

fn load_scene_impl(path: &Path) -> Result<SceneDescription, Box<dyn Error + Send + Sync>> {
    let file = File::open(path)?;

    Ok(match path.extension().and_then(|ext| ext.to_str()).unwrap_or("") {
        "json" => serde_json::from_reader(file)?,
        format => return Err(format!("Unknown scene format {:?}", format).into()),
    })
}
