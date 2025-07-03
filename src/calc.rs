use crate::mania;
use crate::mania::models::pattern::Pattern;
use eyre;
use rosu_map::section::general::GameMode::Mania;
use rosu_map::Beatmap;
use std::collections::HashMap;
pub fn get_patterns(path: &str) -> eyre::Result<HashMap<Pattern, f64>> {
    let map = rosu_map::from_path::<Beatmap>(&path)?;
    match map.mode
    {
        Mania => {
            Ok(mania::transformers(map))
        },
        _ => Err(eyre::eyre!("Unsupported game mode for now."))
    }
}
