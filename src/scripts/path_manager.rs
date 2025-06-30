use std::{ fs::read_to_string, io::{ self, Write } };
use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
pub struct Paths {
   pub discord_path: String,
   pub zapret_path: String,
}

pub fn get_paths_from_file() -> Result<Paths, Box<dyn std::error::Error>> {
   let serialized = std::fs::read_to_string("./src/paths.json")?;
   let deserialized: Paths = serde_json::from_str(&serialized)?;
   Ok(deserialized)
}

// pub fn get_paths() { //-> Result<Paths, io::Error>
//    // let mut paths = Paths {
//    //    discord: String::new(),
//    //    zapret: String::new(),
//    // };

//    get_paths_from_file();

//    // fn get_path(promt: &str) -> Result<String, io::Error> {
//    //    print!("{}", promt);
//    //    io::stdout().flush()?;

//    //    let mut input = String::new();
//    //    io::stdin().read_line(&mut input)?;

//    //    Ok(input.trim().to_string())
//    // }

//    // paths.discord = get_path("Enter Discord's GLOBAL path: ")?;
//    // paths.discord = get_path("Enter Zapret's GLOBAL path: ")?;
//    // paths.zapret = r"C:\Users\Iliya\Desktop\test.txt".to_string(); // for test

//    // Ok(paths)
// }
