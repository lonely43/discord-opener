use crate::{config::is_path_exist, models::Paths};
use std::io::{self, Write};

pub fn prompt_paths() -> Result<Paths, io::Error> {
    let mut paths = Paths {
        discord_path: String::new(),
        zapret_path: String::new(),
    };

    fn prompt_path(prompt: &str) -> Result<String, io::Error> {
        print!("{}", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().replace("\"", "").to_string())
    }

    fn get_path(prompt: &str) -> Result<String, io::Error> {
        Ok(loop {
            let path = prompt_path(prompt)?;
            if is_path_exist(&path)? {
                break path;
            }

            println!("Path doesn't exist: {path}");
            println!("Please repeat.");
        })
    }

    paths.discord_path = get_path("Enter Discord's GLOBAL path: ")?;
    paths.zapret_path = get_path("Enter Zapret's GLOBAL path: ")?;

    Ok(paths)
}
