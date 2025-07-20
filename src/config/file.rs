use std::fs;
use crate::models::Paths;
use serde_json::{ self, Value };

pub fn get_paths_from_file() -> Result<Paths, Box<dyn std::error::Error>> {
   let serialized = std::fs::read_to_string("./src/paths.json")?;
   let deserialized: Paths = serde_json::from_str(&serialized)?;

   let json_value: Value = serde_json::to_value(&deserialized)?;
   is_paths_exist(json_value);

   Ok(deserialized)
}

pub fn set_paths_to_file(paths: &Paths) -> std::io::Result<()> {
   let serialized = serde_json::to_string_pretty(&paths)?;
   fs::write("./src/paths.json", serialized)?;
   Ok(())
}

pub fn is_paths_exist(json_value: Value) {
   if let Value::Object(map) = json_value {
      for (key, value) in map {
         let v: String = value.to_string().replace("\"", "");

         match fs::exists(&v) {
            Ok(true) => println!("{key}: \"{v}\" - exist"),
            Ok(false) => println!("{key}: \"{v}\" - isn't exist"),
            Err(e) => println!("{e}"),
         }
      }
   }
}
