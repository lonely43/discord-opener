use crate::models::Paths;
use serde_json::{self, Value};
use std::fs;

pub fn get_paths_from_file() -> Result<Paths, Box<dyn std::error::Error>> {
    let serialized = std::fs::read_to_string("./src/paths.json")?;
    let deserialized: Paths = serde_json::from_str(&serialized)?;

    let json_value: Value = serde_json::to_value(&deserialized)?;

    if let Value::Object(map) = json_value {
        for (_key, value) in map {
            let path: String = value.to_string().trim().replace("\"", "");

            if !is_path_exist(&path)? {
                return Err("Invalid path found".into());    
            }
        }
    }

    Ok(deserialized)
}

pub fn set_paths_to_file(paths: &Paths) {
    let serialized = serde_json::to_string_pretty(&paths).unwrap();
    fs::write("./src/paths.json", serialized).expect("error in writing paths");
}

pub fn is_path_exist(path: &str) -> Result<bool, std::io::Error> {
    match fs::exists(path) {
        Ok(true) => Ok(true),
        Ok(false) => Ok(false),
        Err(e) => Err(e),
    }
}
