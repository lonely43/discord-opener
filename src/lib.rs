use crate::models::Paths;
use crate::config::{get_paths_from_file, prompt_paths};

pub fn get_paths() -> Result<Paths, std::io::Error> {
   match get_paths_from_file() {
      Ok(paths) => Ok(paths),
      Err(_e) => prompt_paths(),
   }
}