extern crate diploengine;

use diploengine::map::{Map, SpaceType};

#[test]
fn test_default_map_has_seven_nations() {
    let default_map = Map::default();
    assert_eq!(7, default_map.nations.len())
}

#[test]
fn test_default_map_has_seventy_five_spaces() {
    let default_map = Map::default();
    assert_eq!(75, default_map.spaces.len())
}

#[test]
fn test_default_map_has_fifty_six_land_spaces() {
    let default_map = Map::default();
    assert_eq!(56, default_map.spaces.iter()
        .filter(|s| s.space_type == SpaceType::Land)
        .count())
}

#[test]
fn test_default_map_has_nineteen_water_spaces() {
    let default_map = Map::default();
    assert_eq!(19, default_map.spaces.iter()
        .filter(|s| s.space_type == SpaceType::Water)
        .count())
}