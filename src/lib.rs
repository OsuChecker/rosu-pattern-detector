mod structs;
mod mania;
mod std;
mod taiko;

use ::std::error::Error;
use pyo3::prelude::*;
use reqwest::blocking;
use rosu_map;
use rosu_map::Beatmap;

pub fn download_file(url: &str) -> Result<String, Box<dyn Error>> {
    let response = blocking::get(url)?;
    if response.status().is_success() {
        Ok(response.text()?)
    } else {
        Err(format!("HTTP Error: {}", response.status()).into())
    }
}

#[pyfunction]
fn get_map(path: &str) -> PyResult<String> {
    print!("{:?}", path);
    let map = rosu_map::from_path::<Beatmap>(&path).unwrap();
    if (map.mode == rosu_map::section::general::GameMode::Mania) {
        let result_json = crate::mania::transformers(map);
        Ok(result_json.to_string())
    } else if (map.mode == rosu_map::section::general::GameMode::Osu)
    {
        let result_json = crate::std::transformers(map);
        Ok(result_json.to_string())
    } else {
        match map.mode{
            rosu_map::section::general::GameMode::Mania => Ok("Mania".to_string()),
            rosu_map::section::general::GameMode::Osu => Ok("Osu".to_string()),
            rosu_map::section::general::GameMode::Taiko => Ok("Taiko".to_string()),
            rosu_map::section::general::GameMode::Catch => Ok("Catch".to_string()),
        }
    }
}

#[pymodule]
fn pdetector(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_map, m)?)?;
    Ok(())
}
