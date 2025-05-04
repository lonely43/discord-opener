use std::process::Command;

#[path ="./scripts/get_paths.rs"]
mod paths_mod; // get paths of apps

use paths_mod::paths_manager;

fn main() {
    //let paths = paths_manager::get_paths().unwrap(); // make error handler 

    let file = Command::new(r"C:\Users\UglyASF\Desktop\test.txt")
        .status()
        .expect("smth gone wrong");

    println!("hehe: {}", file)
}
