extern crate diploengine;

use diploengine::map::Map;

#[test]
fn test_default_map_has_seven_nations() {
    let default_map = Map::default();
    assert_eq!(7, default_map.nations.len())
}
