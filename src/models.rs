use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct Paths {
   pub discord_path: String,
   pub zapret_path: String,
}