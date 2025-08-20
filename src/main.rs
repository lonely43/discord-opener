use std::{ process::Command, thread, time::Duration };

use crate::app::Paths;

mod prompt;
mod app;

fn main() {
    let paths: Paths = Paths::get().unwrap_or_else(|err| panic!("Failed to get paths: {err}"));

    let mut processes = vec![
        Command::new(paths.zapret_path)
            .spawn()
            .unwrap_or_else(|err| panic!("Failed to open Zapret: {err}")),
        Command::new(paths.discord_path)
            .spawn()
            .unwrap_or_else(|err| panic!("Failed to open Discord: {err}")),
    ];

    while !processes.is_empty() {
        // magic
        processes.retain_mut(|process| !matches!(process.try_wait(), Ok(Some(_))));

        thread::sleep(Duration::from_millis(50));
    }
}

