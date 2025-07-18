use std::io::{ self, Write };

use crate::scripts::write_read_json::get_paths_from_file;
use crate::models::Paths;

fn prompt_paths() -> Result<Paths, io::Error> {
   let mut paths = Paths {
      discord_path: String::new(),
      zapret_path: String::new(),
   };

   fn prompt_path(prompt: &str) -> Result<String, io::Error> {
      print!("{}", prompt);
      io::stdout().flush()?;

      let mut input = String::new();
      io::stdin().read_line(&mut input)?;

      Ok(input.trim().to_string())
   }

   paths.discord_path = prompt_path("Enter Discord's GLOBAL path: ")?;
   paths.zapret_path = prompt_path("Enter Zapret's GLOBAL path: ")?;
   //paths.zapret_path  = r"C:\Users\Iliya\Desktop\test.txt".to_string(); // for test

   Ok(paths)
}

pub fn get_paths() -> Result<Paths, io::Error> {
   match get_paths_from_file() {
      Ok(paths) => Ok(paths),
      Err(_e) => prompt_paths(),
   }
}
