use rosu_pattern_detector::{mania::transform_to_hit_objects};
use rosu_map::Beatmap;

fn main(){
    let path = include_str!("../resources/mania/test_mania_dense_cj.osu");
    let map = rosu_map::from_str::<Beatmap>(&path).unwrap();
    println!("Map: {} - {}", map.title, map.version);
    let patterns = transform_to_hit_objects(map);
    patterns.get_patterns_values().ordered_print();
}