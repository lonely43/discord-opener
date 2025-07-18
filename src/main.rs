mod models;
mod scripts;

use crate::models::Paths;
use crate::scripts::path_manager::get_paths;

fn main() {
   init();
}

pub fn init() {
   match get_paths() {
      Ok(paths) => start(&paths),
      Err(e) => eprintln!("Failed to get paths: {}", e),
   }
}

fn start(paths: &Paths) {
   std::process::Command
      ::new("cmd")
      .args(["/C", "start", &paths.zapret_path])
      .spawn()
      .expect("Failed to open file");

   std::process::Command
      ::new("cmd")
      .args(["/C", "start", &paths.discord_path])
      .spawn()
      .expect("Failed to open file");
}
