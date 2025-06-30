// use crate::scripts::path_manager::{ get_paths, Paths };

// pub fn init() {
//    match get_paths() {
//       Ok(paths) => start(&paths),
//       Err(e) => eprintln!("Failed to get paths: {}", e),
//    }
// }

// fn start(paths: &Paths) {
//    std::process::Command::new("cmd")
//       .args(["/C", "start", &paths.zapret])
//       .spawn()
//       .expect("Failed to open file");
// }
