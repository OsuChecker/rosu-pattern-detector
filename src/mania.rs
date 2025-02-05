use std::cmp::{max, PartialEq};
use std::collections::{BTreeMap, HashMap};
use rosu_map;
use rosu_map::section::hit_objects::{HitObject, HitObjectKind};
use rosu_map::section::timing_points::TimingPoint;
use std::hash::Hash;
use std::fmt;
use serde_json::Value::Null;
use crate::mania::TertiaryPattern::{ANCHOR_JS, JS, LIGHT_JS};

#[derive(Debug, Clone)]
pub struct Notes {
    timestamp: i32,
    notes: Vec<bool>, // Vecteur de bool pour gérer un nombre variable de notes
    pattern: BasePattern, // Utilise l'enum Pattern
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BasePattern {
    Single,
    Jump,
    Hand,
    Quad,
    Chord,
    None, // Cas où il n'y a aucune note (juste pour la couverture, improbable ici)
}

#[derive(Debug, PartialEq, Clone)]
pub enum SecondaryPattern{
    Jack,
    Handstream,
    Jumpstream,
    Singlestream,
    None
}
#[derive(Debug, Hash, PartialEq, Ord, Eq, Clone, PartialOrd)]
pub enum TertiaryPattern{
    DENSE_CHORDJACK,
    CHORDJACK,
    SPEEDJACK,
    CHORDSTREAM,
    LIGHT_JS,
    ANCHOR_JS,
    JS,
    None
}

#[derive(Debug)]
pub struct Measure {
    start_time: i32,
    notes: Vec<Notes>,
    secondary_pattern : SecondaryPattern,
    tertiary_pattern: TertiaryPattern,
    npm : i32
}
impl Measure {
    fn notes(&self) -> &Vec<Notes> {
        &self.notes
    }
}
impl Measure {
    pub fn print_notes(&self) {
        for note in &self.notes {
            let line = note.to_display_string();
            println!("{}", line);
        }
    }
}

impl Notes {
    pub fn to_display_string(&self) -> String {
        self.notes
            .iter()
            .map(|&active| if active { 'O' } else { 'X' })
            .collect()
    }
}

impl fmt::Display for TertiaryPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TertiaryPattern::DENSE_CHORDJACK => write!(f, "Dense Chordjack"),
            TertiaryPattern::CHORDJACK => write!(f, "Chordjack"),
            TertiaryPattern::SPEEDJACK => write!(f, "Speedjack"),
            TertiaryPattern::CHORDSTREAM => write!(f, "ChordStream"),
            TertiaryPattern::LIGHT_JS => write!(f, "Light JS"),
            TertiaryPattern::ANCHOR_JS => write!(f, "Anchor JS"),
            TertiaryPattern::JS => write!(f, "JS"),
            TertiaryPattern::None => write!(f, "None"),
        }
    }
}

impl fmt::Display for SecondaryPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SecondaryPattern::Jumpstream => write!(f, "JS"),
            SecondaryPattern::Jack => write!(f, "Jack"),
            SecondaryPattern::Handstream => write!(f, "HS"),
            SecondaryPattern::Singlestream => write!(f, "SS"),
            SecondaryPattern::None => write!(f, "None"),
        }
    }
}

fn detect_pattern(note: &Notes) -> BasePattern {
    // Compter combien de `true` il y a parmi les notes
    let count = note.notes.iter().filter(|&&n| n).count();

    // Fonction interne pour obtenir le pattern correspondant
    fn get_pattern(number: usize) -> BasePattern {
        match number {
            1 => BasePattern::Single,
            2 => BasePattern::Jump,
            3 => BasePattern::Hand,
            4 => BasePattern::Quad,
            _ => BasePattern::Chord, // Pour 5 ou plus, retourne Chord
        }
    }

    get_pattern(count)
}


pub(crate) fn transform_ho_to_mania_notes(hO: Vec<HitObject>, num_keys: usize) -> Vec<Notes> {
    let mut notes: Vec<Notes> = Vec::new();

    // Définir les positions des touches en fonction du nombre de touches (4K ou 7K)
    let positions = match num_keys {
        4 => vec![64f32, 192f32, 320f32, 448f32], // 4K (quatre touches)
        7 => vec![36f32, 109f32, 182f32, 256f32, 329f32, 402f32, 475f32], // 7K (sept touches)
        _ => return notes, // Retourner une liste vide pour des configurations non supportées
    };

    for hO in hO {
        // Obtenez la position basée sur le type d'objet
        let pos_x = match &hO.kind {
            HitObjectKind::Circle(circle) => circle.pos.x,
            HitObjectKind::Slider(slider) => slider.pos.x,
            _ => continue, // Ignorer les autres types d'objets
        };

        let timestamp = hO.start_time as i32;

        // Trouver l'indice correspondant dans les positions des touches
        if let Some(index) = positions.iter().position(|&x| x == pos_x) {
            // Créer un vecteur de bools en fonction de la position
            let mut new_note = vec![false; num_keys]; // Initialiser avec `false`
            new_note[index] = true; // Marquer la note active à l'index correspondant

            // Recherchez une note au même timestamp
            if let Some(existing_note) = notes.iter_mut().find(|note| note.timestamp == timestamp) {
                // Mettre à jour les colonnes actives
                for i in 0..num_keys {
                    existing_note.notes[i] |= new_note[i];
                }
                existing_note.pattern = detect_pattern(existing_note); // Met à jour le pattern
            } else {
                // Sinon, créer une nouvelle note
                notes.push(Notes {
                    timestamp,
                    notes: new_note.clone(), // Clone pour éviter de déplacer new_note
                    pattern: detect_pattern(&Notes {
                        timestamp,
                        notes: new_note.clone(),
                        pattern: BasePattern::None,
                    }),
                });
            }
        }
    }

    notes
}


pub(crate) fn analyze_patterns_by_measures_advanced(
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

    // Puissance utilisée pour amplifier les poids
    let amplification_power: f64 = 1.0;

    // Étape 2 : Analyse des mesuresp0
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
        let mut prev_notes : Notes ;
        let vec_jack: Vec<i32> = vec![0; measure.notes[0].notes.len()];
        for (i, note) in measure.notes.iter().enumerate() {
            if i > 0 {
                let prev = &measure.notes[i - 1];
                if note.notes.iter().zip(prev.notes.iter()).any(|(n, p)| *n && *p) {
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

pub(crate) fn group_notes_by_measures(notes: Vec<Notes>, timing_points: Vec<TimingPoint>) -> BTreeMap<i32, Measure> {
    let mut measures: BTreeMap<i32, Measure> = BTreeMap::new();

    for note in notes {
        // Trouver le timing point actif pour la note
        let timing_point = timing_points
            .iter()
            .rev()
            .find(|tp| note.timestamp >= tp.time as i32)
            .unwrap_or_else(|| timing_points.first().expect("La liste des timing points ne doit pas être vide"));

        // Calculer la mesure correspondante
        let beat_len: f32 = timing_point.beat_len as f32; // Assurez-vous que beat_len est un f32
        let start_time: i32 = timing_point.time as i32; // start_time est un entier (i32)

        let measure_idx = ((note.timestamp - start_time) as f32 / beat_len).floor() as i32;
        let measure_start_time = start_time + (measure_idx as f32 * beat_len) as i32;

        // Ajouter la note dans sa mesure correspondante
        let measure_entry = measures.entry(measure_start_time).or_insert_with(|| Measure {
            start_time: measure_start_time,
            notes: Vec::new(),
            secondary_pattern: SecondaryPattern::None,
            tertiary_pattern: TertiaryPattern::None,
            npm: 0, // Initialisation du compteur de notes par mesure
        });

        // Ajouter la note à la mesure
        measure_entry.notes.push(note.clone());

        // Calculer le nombre de "notes actives" pour ajouter au compteur NPM
        let active_notes = note.notes.iter().filter(|&&n| n).count() as i32;

        // Ajouter au total des notes par mesure (NPM)
        measure_entry.npm += active_notes;
    }

    measures
}

pub(crate) fn analyze_patterns_tertiary(
    grouped_measures: &mut BTreeMap<i32, Measure>, key : i32
) -> BTreeMap<TertiaryPattern, f64> {
    let mut map : BTreeMap<TertiaryPattern,f64> =  BTreeMap::new();
    // Étape 1 : Calculer le NPM moyen
    let measure_count = grouped_measures.len();
    let average_npm = grouped_measures
        .values()
        .map(|measure| measure.npm as f64)
        .sum::<f64>()
        / if measure_count > 0 { measure_count as f64 } else { 1.0 };

    // Puissance utilisée pour amplifier les poids
    let amplification_power: f64 = 1.0;

    // Étape 2 : Analyse des mesures
    for measure in grouped_measures.values_mut() {
        // Calculer la pondération actuelle avec amplification si average_npm > 0.0
        let density_factor = if average_npm > 0.0 {
            (measure.npm as f64 / average_npm)*0.8
        } else {
            1.0 // Si le NPM moyen est zéro, densité neutre
        };

        // Appliquer le facteur de pondération basé sur la densité
        if measure.secondary_pattern == SecondaryPattern::Jack {
            let key = check_jack(measure);
            measure.tertiary_pattern = key.clone();
            *map.entry(key).or_insert(0.0) += density_factor;

        }
        else if measure.secondary_pattern == SecondaryPattern::Jumpstream
        {
            let key = check_js(measure);
            measure.tertiary_pattern = key.clone();
            *map.entry(key).or_insert(0.0) += density_factor;   
        }
    }
    map
}

fn check_jack(p0: &mut Measure) -> TertiaryPattern {
    let mut pattern_count: HashMap<BasePattern, usize> = HashMap::new();

    for note in p0.notes() {
        *pattern_count.entry(note.pattern.clone()).or_insert(0) += 1;
    }
    let single = *pattern_count.get(&BasePattern::Single).unwrap_or(&0);
    let jump = *pattern_count.get(&BasePattern::Jump).unwrap_or(&0);
    let hand = *pattern_count.get(&BasePattern::Hand).unwrap_or(&0);
    let quad = *pattern_count.get(&BasePattern::Quad).unwrap_or(&0);
    let chord = *pattern_count.get(&BasePattern::Chord).unwrap_or(&0);

    if hand > jump
    {
        crate::mania::TertiaryPattern::DENSE_CHORDJACK
    }
    else if quad > 0 && jump + hand + quad > single
    {
        crate::mania::TertiaryPattern::CHORDJACK
    }
    else {
        check_jackspeed_or_chordstream(p0)
    }

}

fn check_jackspeed_or_chordstream(measure: &mut Measure) -> TertiaryPattern{

    let mut jack_count = 0; // Nouveau compteur

    for (i, note) in measure.notes.iter().enumerate() {
        if i > 0 {
            let prev = &measure.notes[i - 1]; // Obtenir la note précédente
            if note.notes.iter().zip(prev.notes.iter()).any(|(n, p)| *n && *p) {
                jack_count += 1; // Incrémenter lorsqu'on détecte un "jack"
            }
        }
    }
    if jack_count <=1
    {
        crate::mania::TertiaryPattern::CHORDSTREAM
    }
    else
    {
        crate::mania::TertiaryPattern::SPEEDJACK
    }
}

fn check_js(measure: &mut Measure) -> TertiaryPattern {
    let mut pattern_count: HashMap<BasePattern, usize> = HashMap::new();

    for note in measure.notes() {
        *pattern_count.entry(note.pattern.clone()).or_insert(0) += 1;
    }
    let single = *pattern_count.get(&BasePattern::Single).unwrap_or(&0);
    let jump = *pattern_count.get(&BasePattern::Jump).unwrap_or(&0);

        let mut vect_int = vec![0; measure.notes[0].notes.len()];

        for (i, note) in measure.notes.iter().enumerate() {
            for (i, &is_active) in note.notes.iter().enumerate() {
                if is_active {
                    vect_int[i] += 1;
                }
            }
        }

        println!("{:?}", vect_int);
    if let Some(&max_value) = vect_int.iter().max() {
        if max_value > 3 {
            return TertiaryPattern::ANCHOR_JS;
        } else if jump < single {
            TertiaryPattern::LIGHT_JS
        } else {
            TertiaryPattern::JS
        }
    }
    else{
        JS
    }
}


