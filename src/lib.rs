use std::cmp::PartialEq;
use std::collections::{BTreeMap, HashMap};
use std::iter::Map;
use pyo3::prelude::*;
use rosu_map;
use rosu_map::Beatmap;
use rosu_map::section::hit_objects::{HitObject, HitObjectKind};
use rosu_map::section::timing_points::TimingPoint;
use reqwest::blocking;
use std::error::Error;
use serde_json::Value::Null;

/// Module principal à exposer pour Python

#[derive(Debug, Clone)]
struct Notes {
    timestamp: i32,
    note_1: bool,
    note_2: bool,
    note_3: bool,
    note_4: bool,
    pattern: BasePattern, // Utilise l'enum Pattern
}


#[derive(Debug, PartialEq, Clone)]
enum BasePattern {
    Single,
    Jump,
    Hand,
    Quad,
    None, // Cas où il n'y a aucune note (juste pour la couverture, improbable ici)
}

#[derive(Debug, PartialEq, Clone)]
enum  SecondaryPattern{
    Jack,
    Handstream,
    Jumpstream,
    Singlestream,
    None
}
#[derive(Debug)]
struct Measure {
    start_time: i32,
    notes: Vec<Notes>,
    secondary_pattern : SecondaryPattern,
    npm : i32
}


fn detect_pattern(note: &Notes) -> BasePattern {
    // Compter combien de `true` il y a parmi les notes
    let count = [
        note.note_1,
        note.note_2,
        note.note_3,
        note.note_4,
    ]
        .iter()
        .filter(|&&n| n) // Garder les `true`
        .count();

    // Renvoyer le Pattern correspondant
    match count {
        1 => BasePattern::Single,
        2 => BasePattern::Jump,
        3 => BasePattern::Hand,
        4 => BasePattern::Quad,
        _ => BasePattern::None, // Cas par défaut
    }
}

fn transform_ho_to_mania_notes(hO: Vec<HitObject>) -> Vec<Notes> {
    let mut notes: Vec<Notes> = Vec::new();

    for hO in hO {
        // Obtenez la position basée sur le type d'objet
        let pos_x = match &hO.kind {
            HitObjectKind::Circle(circle) => circle.pos.x,
            HitObjectKind::Slider(slider) => slider.pos.x,
            _ => continue, // Ignorer les autres types d'objets
        };

        let timestamp = hO.start_time as i32;

        // Détermine quelle colonne est active
        let new_note = match pos_x {
            64f32 => (true, false, false, false),
            192f32 => (false, true, false, false),
            320f32 => (false, false, true, false),
            448f32 => (false, false, false, true),
            _ => continue, // Ignorer les positions non valides
        };

        // Recherchez une note au même timestamp
        if let Some(existing_note) = notes.iter_mut().find(|note| note.timestamp == timestamp) {
            // Mettre à jour les colonnes actives
            existing_note.note_1 |= new_note.0;
            existing_note.note_2 |= new_note.1;
            existing_note.note_3 |= new_note.2;
            existing_note.note_4 |= new_note.3;
            existing_note.pattern = detect_pattern(existing_note); // Met à jour le pattern
        } else {
            // Sinon, créer une nouvelle note
            notes.push(Notes {
                timestamp,
                note_1: new_note.0,
                note_2: new_note.1,
                note_3: new_note.2,
                note_4: new_note.3,
                pattern: detect_pattern(&Notes {
                    timestamp,
                    note_1: new_note.0,
                    note_2: new_note.1,
                    note_3: new_note.2,
                    note_4: new_note.3,
                    pattern: BasePattern::None,
                }),
            });
        }
    }

    notes
}
fn analyze_patterns_by_measures_advanced(
    grouped_measures: &mut BTreeMap<i32, Measure>,
) -> (f64, f64, f64, f64) {
    let mut jack_count = 0.0;
    let mut jumpstream_count = 0.0;
    let mut singlestream_count = 0.0;
    let mut handstream_count = 0.0;

    // Étape 1 : Calculer le NPM moyen
    let measure_count = grouped_measures.len();
    let average_npm = grouped_measures
        .values()
        .map(|measure| measure.npm as f64)
        .sum::<f64>()
        / if measure_count > 0 { measure_count as f64 } else { 1.0 };

    println!("NPM moyen : {}", average_npm);

    // Puissance utilisée pour amplifier les poids
    let amplification_power: f64 = 1.0;

    // Étape 2 : Analyse des mesures
    for measure in grouped_measures.values_mut() {
        // Calculer la pondération actuelle avec amplification si average_npm > 0.0
        let weight = if average_npm > 0.0 {
            (measure.npm as f64 / average_npm).powf(amplification_power)
        } else {
            1.0 // Si le NPM moyen est zéro, pas de pondération spéciale
        };

        let mut has_jack = false;
        let mut has_jumpstream = false;
        let mut has_singlestream = false;
        let mut has_handstream = false;

        // Parcourir les notes de la mesure
        for (i, note) in measure.notes.iter().enumerate() {
            // Détecter un Jack (même colonne active dans deux notes consécutives)
            if i > 0 {
                let prev = &measure.notes[i - 1];
                if (note.note_1 && prev.note_1)
                    || (note.note_2 && prev.note_2)
                    || (note.note_3 && prev.note_3)
                    || (note.note_4 && prev.note_4)
                {
                    has_jack = true;
                }
            }

            // Détection des motifs principaux
            match note.pattern {
                BasePattern::Hand => has_handstream = true,
                BasePattern::Jump => has_jumpstream = true,
                BasePattern::Single => has_singlestream = true,
                _ => {}
            }
        }

        // Étape 3 : Ajouter au compte pondéré des patrons avec poids amplifié
        if has_jack {
            jack_count += weight;
            measure.secondary_pattern = SecondaryPattern::Jack;
        } else if has_handstream {
            handstream_count += weight;
            measure.secondary_pattern = SecondaryPattern::Handstream;
        } else if has_jumpstream {
            jumpstream_count += weight;
            measure.secondary_pattern = SecondaryPattern::Jumpstream;
        } else if has_singlestream {
            singlestream_count += weight;
            measure.secondary_pattern = SecondaryPattern::Singlestream;
        }
    }

    (jack_count, jumpstream_count, singlestream_count, handstream_count)
}
fn group_notes_by_measures(notes: Vec<Notes>, timing_points: Vec<TimingPoint>) -> BTreeMap<i32, Measure> {
    let mut measures: BTreeMap<i32, Measure> = BTreeMap::new();

    for note in notes {
        // Trouver le timing point actif pour la note
        let timing_point = timing_points
            .iter()
            .rev()
            .find(|tp| note.timestamp >= tp.time as i32)
            .expect("Un timing point doit correspondre");

        // Calculer la mesure correspondante
        let beat_len: f32 = timing_point.beat_len as f32; // Assurez-vous que beat_len est un f32
        let start_time: i32 = timing_point.time as i32; // `start_time` est un entier (i32)

        let measure_idx = ((note.timestamp - start_time) as f32 / beat_len).floor() as i32;
        let measure_start_time = start_time + (measure_idx as f32 * beat_len) as i32;

        // Ajouter la note dans sa mesure correspondante
        let measure_entry = measures.entry(measure_start_time).or_insert_with(|| Measure {
            start_time: measure_start_time,
            notes: Vec::new(),
            secondary_pattern: SecondaryPattern::None,
            npm: 0, // Initialisation du compteur de notes par mesure
        });

        // Ajouter la note à la mesure
        measure_entry.notes.push(note.clone());

        // Calculer le nombre de "notes actives" pour ajouter au compteur NPM
        let active_notes = note.note_1 as i32
            + note.note_2 as i32
            + note.note_3 as i32
            + note.note_4 as i32;

        // Ajouter au total des notes par mesure (NPM)
        measure_entry.npm += active_notes;
    }

    measures
}


pub fn download_file(url: &str) -> Result<String, Box<dyn Error>> {
    let response = blocking::get(url)?;
    if response.status().is_success() {
        Ok(response.text()?)
    } else {
        Err(format!("HTTP Error: {}", response.status()).into())
    }
}


#[pyfunction]
fn get_map(url : &str) -> PyResult<String>{
    let path = download_file(url).unwrap();
    let map = rosu_map::from_str::<Beatmap>(&path).unwrap();
    let timing_point = map.control_points.timing_points;

    let notes =transform_ho_to_mania_notes(map.hit_objects);
    let mut mesure = group_notes_by_measures(notes, timing_point);
    let (jack_count, jumpstream_count, singlestream_count, handstream_count) =
        analyze_patterns_by_measures_advanced(&mut mesure);

    println!("Jacks: {}", jack_count);
    println!("Jumpstreams: {}", jumpstream_count);
    println!("Single streams: {}", singlestream_count);
    println!("Handstreams: {}", handstream_count);


    Ok("test".parse()?)
}


#[pymodule]
fn pdetector(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_map, m)?)?;
    Ok(())
}


