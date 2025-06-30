mod scripts;
mod app;

use scripts::path_manager::Paths;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   // app::init();
   let paths: Paths = scripts::path_manager::get_paths_from_file()?;

   println!("Discord path: {}", paths.discord_path);
   println!("Zapret path: {}", paths.zapret_path);
   Ok(())
}
