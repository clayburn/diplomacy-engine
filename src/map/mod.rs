use serde::{Deserialize};

#[derive(Deserialize, Debug, PartialEq)]
pub struct Map {
    pub nations: Vec<Nation>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Nation {
    pub name: String,
}

impl Map {
    pub fn default() -> Map {
        let bytes = include_bytes!("default_map.json");
        serde_json::from_slice(bytes).expect("Unable to parse default map")
    }
}
