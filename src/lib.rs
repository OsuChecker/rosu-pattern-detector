mod mania;

use pyo3::prelude::*;
use reqwest::blocking;
use rosu_map;
use rosu_map::section::hit_objects::{HitObject, HitObjectKind};
use rosu_map::section::timing_points::TimingPoint;
use rosu_map::section::Section::Mania;
use rosu_map::Beatmap;
use serde_json::json;
use serde_json::Value::Null;
use std::cmp::PartialEq;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::iter::Map;

/// Module principal à exposer pour Python

pub fn download_file(url: &str) -> Result<String, Box<dyn Error>> {
    let response = blocking::get(url)?;
    if response.status().is_success() {
        Ok(response.text()?)
    } else {
        Err(format!("HTTP Error: {}", response.status()).into())
    }
}

#[pyfunction]
fn get_map(url: &str) -> PyResult<String> {
    let path = download_file(url).unwrap();
    let map = rosu_map::from_str::<Beatmap>(&path).unwrap();
    if (map.mode != rosu_map::section::general::GameMode::Mania) {
        let timing_point = map.control_points.timing_points;

        let notes = mania::transform_ho_to_mania_notes(map.hit_objects);
        let mut mesure = mania::group_notes_by_measures(notes, timing_point);
        let (jack_count, jumpstream_count, singlestream_count, handstream_count) =
            mania::analyze_patterns_by_measures_advanced(&mut mesure);

        // Convertir les résultats en JSON
        let result_json = json!({
            "jack": jack_count,
            "jumpstream": jumpstream_count,
            "singlestream": singlestream_count,
            "handstream": handstream_count
        });
    } else {
        return Ok("".to_string());
    }
    Ok(result_json.to_string())
}

#[pymodule]
fn pdetector(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_map, m)?)?;
    Ok(())
}
