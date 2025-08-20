use anyhow::Result;
use std::{ io::{ stdin, stdout, Write }, path::PathBuf };

use crate::app::Paths;

fn ask_path(name: &str) -> Result<PathBuf> {
   loop {
      print!("Enter {name}'s global path: ");
      stdout().flush();

      let mut input = String::new();
      stdin().read_line(&mut input);

      let path = PathBuf::from(input.trim());

      if path.exists() {
         break Ok(path);
      }
   }
}

pub fn onboarding() -> Result<Paths> {
   Ok(Paths {
      discord_path: ask_path("Discord")?,
      zapret_path: ask_path("Zapret")?,
   })
}
