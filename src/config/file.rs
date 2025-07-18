use crate::models::Paths;
use serde_json;

pub fn get_paths_from_file() -> Result<Paths, Box<dyn std::error::Error>> {
   let serialized = std::fs::read_to_string("./src/paths.json")?;
   let deserialized: Paths = serde_json::from_str(&serialized)?;
   Ok(deserialized)
}