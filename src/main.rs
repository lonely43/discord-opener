#[path ="./scripts/get_paths.rs"]
mod paths_mod; // get paths of apps

//use paths_mod::paths_manager;

fn main() {
    //let paths = paths_manager::get_paths().unwrap(); // make error handler 

    std::process::Command::new("cmd")
        .args(["/C", "start", r"C:\Users\UglyASF\Desktop\test.txt"])
        .spawn()
        .expect("file opening failed");
}
