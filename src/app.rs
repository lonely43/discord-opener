use std::{ path::PathBuf, fs::File };

use anyhow::{ Context, Ok, Result };
use serde::{ Deserialize, Serialize };

use crate::prompt::onboarding;

#[derive(Serialize, Deserialize)]
pub struct Paths {
   pub discord_path: PathBuf,
   pub zapret_path: PathBuf,
}

impl Paths {
   fn get_config_path() -> Result<PathBuf> {
      Ok(
         std::env
            ::home_dir()
            .with_context(|| "no user's home folder found.".to_string())?
            .join("discord-opener-paths.json")
      )
   }

   pub fn get() -> Result<Self> {
      let config_path = Self::get_config_path()?;

      if config_path.exists() {
         let file = File::open(config_path)?;
         let paths: Paths = serde_json::from_reader(file)?;

         Ok(paths)
      } else {
        let paths: Paths = onboarding()?;

        let file = File::create(config_path)?;
        serde_json::to_writer_pretty(file, &paths)?;

        Ok(paths)
      }
   }
}