use serde::{Deserialize};

#[derive(Deserialize, Debug, PartialEq)]
pub struct Map {
    pub nations: Vec<Nation>,
    pub spaces: Vec<Space>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Nation {
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Space {
    pub long_name: String,
    pub short_name: String,
    pub space_type: SpaceType,
    pub supply_center: Option<SupplyCenter>
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SupplyCenter {
    pub nation: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum SpaceType {
    Land, Water,
}

impl Map {
    pub fn default() -> Map {
        let bytes = include_bytes!("default_map.json");
        serde_json::from_slice(bytes).expect("Unable to parse default map")
    }
}
