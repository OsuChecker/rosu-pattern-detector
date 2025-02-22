use crate::mania;
use crate::mania::structs::SecondaryPattern;
use eyre;
use rosu_map::section::general::GameMode::Mania;
use rosu_map::Beatmap;
use std::collections::HashMap;
pub(crate) fn get_patterns(path: &str) -> eyre::Result<HashMap<SecondaryPattern, f64>> {
    let map = rosu_map::from_path::<Beatmap>(&path)?;
    match map.mode
    {
        Mania => {
            Ok(mania::transformers(map))
        },
        _ => Err(eyre::eyre!("Unsupported game mode for now."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mania::structs::JumpstreamPattern::JS;
    use crate::mania::structs::SinglestreamPattern;
    use crate::mania::structs::{JackPattern, JumpstreamPattern};
    use crate::utils::{max_by_secondary_type, max_values};

    #[test]
    fn test_get_patterns_mania_secondary_js() {
        let test_map_path = "./resources/mania/test_mania_js.osu";
        let secondary = get_patterns(test_map_path).unwrap();
        let max_vals = max_by_secondary_type(&secondary);
        assert_eq!(SecondaryPattern::Jumpstream(JumpstreamPattern::All), max_vals[0].0);
    }


    #[test]
    fn test_get_patterns_mania_tertiary_js() {
        let test_map_path = "./resources/mania/test_mania_js.osu";
        let secondary = get_patterns(test_map_path).unwrap();
        let max_vals = max_values(&secondary);
        assert_eq!(SecondaryPattern::Jumpstream(JS), max_vals[0].0);
    }

    #[test]
    fn test_get_patterns_mania_secondary_cj() {
        let test_map_path = "./resources/mania/test_mania_dense_cj.osu";
        let secondary = get_patterns(test_map_path).unwrap();
        let max_vals = max_by_secondary_type(&secondary);
        assert_eq!(SecondaryPattern::Jack(JackPattern::All), max_vals[0].0);
    }


    #[test]
    fn test_get_patterns_mania_tertiary_dense_cj() {
        let test_map_path = "./resources/mania/test_mania_dense_cj.osu";
        let secondary = get_patterns(test_map_path).unwrap();
        let max_vals = max_values(&secondary);
        assert!(max_vals.iter().any(|(pattern, _)| matches!(pattern, SecondaryPattern::Jack(JackPattern::DenseChordjack))));
    }


    #[test]
    fn test_get_patterns_mania_secondary_singlestream() {
        let test_map_path = "./resources/mania/test_mania_singlestream.osu";
        let secondary = get_patterns(test_map_path).unwrap();
        let max_vals = max_by_secondary_type(&secondary);
        assert_eq!(SecondaryPattern::Singlestream(SinglestreamPattern::All), max_vals[0].0);
    }


    #[test]
    fn test_get_patterns_mania_tertiary_singlestream() {
        let test_map_path = "./resources/mania/test_mania_singlestream.osu";
        let secondary = get_patterns(test_map_path).unwrap();
        let max_vals = max_values(&secondary);
        assert!(max_vals.iter().any(|(pattern, _)| matches!(pattern, SecondaryPattern::Singlestream(SinglestreamPattern::Singlestream))));
    }

    #[test]
    fn test_get_patterns_invalid_path() {
        let invalid_path = "non_existent_file.osu";

        let result = get_patterns(invalid_path);

        assert!(result.is_err());
    }
}


