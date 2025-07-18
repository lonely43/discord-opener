use serde::{ Deserialize }; //Serialize

#[derive(Deserialize)]
pub struct Paths {
   pub discord_path: String,
   pub zapret_path: String,
}