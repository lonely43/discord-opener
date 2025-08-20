mod models;
mod config;
mod app;

use std::thread;
use std::process::Command;
use std::time::Duration;

use crate::models::Paths;
use crate::app::get_paths;

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
   let mut processes = vec![
      Command::new("cmd")
         .args(["/C", "start", &paths.zapret_path])
         .spawn()
         .expect("Failed to open file"),

      Command::new("cmd")
         .args(["/C", "start", &paths.discord_path])
         .spawn()
         .expect("Failed to open file")
   ];

   while processes.len() > 0 {
      processes.retain_mut(|process| matches!(process.try_wait(), Ok(Some(_))));

      thread::sleep(Duration::from_millis(50))
   }
}
