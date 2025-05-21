#[allow(dead_code)]

pub mod paths_manager {
   use std::io::{self, Write};
   
   pub struct Paths {
      pub discord: String, 
      pub zapret: String
   }

   pub fn get_paths() -> Result<Paths, io::Error> {
      let mut paths = Paths {
         discord: String::new(),
         zapret: String::new()
      };

      fn get_path(promt: &str) -> Result<String, io::Error>{
         print!("{}", promt);
         io::stdout().flush()?;

         let mut input = String::new();
         io::stdin().read_line(&mut input)?;

         Ok(input.trim().to_string())
      }

      paths.discord = get_path("Enter global path for Discord: ")?;
      paths.zapret = get_path("Enter global path for Zapret-Discord: ")?;

      Ok(paths)
   }
}