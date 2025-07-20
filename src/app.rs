use crate::models::Paths;
use crate::config::{get_paths_from_file, set_paths_to_file, prompt_paths};

pub fn get_paths() -> Result<Paths, std::io::Error> {
   match get_paths_from_file() {
      Ok(paths) => Ok(paths),
      Err(_e) => send_paths_to_file(),
   }
}

fn send_paths_to_file() -> Result<Paths, std::io::Error> {
   let paths: Paths = prompt_paths()?;

   set_paths_to_file(&paths);

   Ok(paths)
}