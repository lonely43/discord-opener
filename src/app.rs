#[path ="./scripts/path_manager.rs"]
mod path_manager;

pub mod app {
   use super::path_manager::{get_paths, Paths};

   pub fn init() {
      match get_paths() {
         Ok(paths) => start(&paths),
         Err(e) => eprintln!("Failed to get paths: {}", e),
      }
   }

   pub fn start(paths: &Paths) {
      std::process::Command::new("cmd")
         .args(["/C", "start", &paths.zapret])
         .spawn()
         .expect("Failed to open file");
   }
}