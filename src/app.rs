use crate::config::{get_paths_from_file, is_path_exist, prompt_paths, set_paths_to_file};
use crate::models::Paths;

pub fn get_paths() -> Result<Paths, std::io::Error> {
    match is_path_exist("./src/paths.json") {
        Ok(true) => match get_paths_from_file() {
            Ok(paths) => Ok(paths),
            _ => {
                println!("Failed to get paths from cfg");
                ask_for_paths()
            }
        },
        _ => ask_for_paths(),
    }
}

fn ask_for_paths() -> Result<Paths, std::io::Error> {
    let paths: Paths = prompt_paths()?;

    set_paths_to_file(&paths);

    Ok(paths)
}
