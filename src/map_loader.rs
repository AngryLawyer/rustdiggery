use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde_json::Error;

#[derive(Deserialize, Clone)]
pub struct MapData {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub cells: String,
    pub crystals_to_pass: u32,
}

pub fn load_maps() -> Result<Vec<MapData>, Error> {
    //TODO: Get this from an external file
    let mut file = File::open("./assets/maps.json").expect("Could not load maps.json");
    let mut original_data = String::new();
    file.read_to_string(&mut original_data).expect("Could not read maps.json");
    let maps: Vec<MapData> = serde_json::from_str(&original_data)?;
    Ok(maps)
}
