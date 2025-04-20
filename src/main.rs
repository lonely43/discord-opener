#[path ="./scripts/get_paths.rs"]
mod paths_mod; // get paths of apps

use paths_mod::paths_manager;

fn main() {
    println!("Hello, world!");

    let paths = paths_manager::get_paths();
    print!("{}", paths.unwrap().zapret)
}
