use std::collections::HashMap;
use std::error::Error;
use rosu_map::Beatmap;
use eyre;
use reqwest::blocking;
use rosu_map::section::general::GameMode::{Mania, Taiko, Catch,Osu};
use serde_json::Value;
use crate::mania;

pub(crate) fn get_patterns(path: &str) -> eyre::Result<(HashMap<crate::mania::SecondaryPattern, f64>, HashMap<mania::TertiaryPattern, f64>)> {
    let map = rosu_map::from_path::<Beatmap>(&path)?;
    match map.mode
    {
        Mania => {
            Ok(mania::transformers(map))
        },
        _ => Err(eyre::eyre!("Unsupported game mode."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Map};

    fn max_value(map: &Map<String, Value>) -> (String, f64) {
        let max_pair = map.iter()
            .max_by(|a, b| a.1.as_f64().unwrap().partial_cmp(&b.1.as_f64().unwrap()).unwrap())
            .unwrap();

        (max_pair.0.to_string(), max_pair.1.as_f64().unwrap())
    }

    #[test]
    fn test_get_patterns_mania_secondary_js() {
        let test_map_path = "./resources/mania/test_mania_js.osu";
        let result = get_patterns(test_map_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        let secondary = result.get("SecondaryPattern").unwrap().as_object().unwrap();
        let max_pattern = max_value(&secondary);
        assert_eq!(max_pattern.0, "Jumpstream");
    }
    #[test]
    fn test_get_patterns_mania_tertiary_js() {
        let test_map_path = "./resources/mania/test_mania_js.osu";
        let result = get_patterns(test_map_path);
        assert!(result.is_ok());
        let result = result.unwrap();
        let tertiary= result.get("TertiaryPattern").unwrap().as_object().unwrap();
        let max_pattern = max_value(&tertiary);
        assert_eq!(max_pattern.0, "JS");
    }
    /*
    #[test]
    fn test_get_patterns_unsupported_mode() {
        let test_map_path = "./resources/test_mania_js.osu";

        let result = get_patterns(test_map_path);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Unsupported game mode."
        );
    }
    */
    #[test]
    fn test_get_patterns_invalid_path() {
        let invalid_path = "non_existent_file.osu";

        let result = get_patterns(invalid_path);

        assert!(result.is_err());
    }
}


