use serde_json;
use serde_json::Error;

#[derive(Deserialize, Clone)]
pub struct MapData {
    pub name: String,
    pub width: u32,
    pub height: u32
}

pub fn load_maps() -> Result<Vec<MapData>, Error> {
    //TODO: Get this from an external file
    let original_data = r#"[{
        "name": "Introduction",
        "width": 8,
        "height": 8
    }, {
        "name": "Level 1",
        "width": 16,
        "height": 16
    }]"#;

    let maps: Vec<MapData> = serde_json::from_str(original_data)?;
    Ok(maps)
}
