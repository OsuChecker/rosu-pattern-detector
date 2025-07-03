use crate::structs::CommonMeasure;
use crate::mania::models::pattern::Pattern;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Notes {
    Single,
    Jump,
    Hand,
    Quad,
    Chord,
    None
}

#[derive(Debug, Clone)]
pub struct NotesStruct {
    pub(crate) timestamp: i32,
    pub(crate) notes: Vec<bool>,
}

impl NotesStruct {
    pub fn to_display_string(&self) -> String {
        let notes_str: String = self.notes
            .iter()
            .map(|&active| if active { 'O' } else { 'X' })
            .collect();
        format!("{}: {}", self.timestamp, notes_str)
    }
    pub fn get_pattern(&self) -> Notes {
        let count = self.notes.iter().filter(|&&n| n).count();
        match count {
            1 => Notes::Single,
            2 => Notes::Jump,
            3 => Notes::Hand,
            4 => Notes::Quad,
            _ => Notes::Chord,
        }
    }
}

#[derive(Debug)]
pub struct ManiaMeasure {
    pub(crate) measure: CommonMeasure,
    pub(crate) notes: Vec<NotesStruct>,
    pub(crate) pattern: Pattern,
    pub(crate) value: f64,
}

impl ManiaMeasure {
    pub fn detect_pattern(&mut self) -> Pattern {
        if self.has_jack_pattern() {
            Pattern::Jack(self.determine_jack_type())
        } else if self.has_hand_notes() {
            Pattern::Handstream(self.determine_handstream_type())
        } else if self.has_jump_notes() {
            Pattern::Jumpstream(self.determine_jumpstream_type())
        } else if self.has_single_notes() {
            Pattern::Singlestream(crate::mania::models::pattern::SinglestreamPattern::Singlestream)
        } else {
            Pattern::None
        }
    }

    fn has_jack_pattern(&self) -> bool {
        self.notes.windows(2).any(|w| {
            w[0].get_pattern() == Notes::Single && w[1].get_pattern() == Notes::Jump
        })
    }

    fn has_hand_notes(&self) -> bool {
        self.notes.iter().any(|n| n.get_pattern() == Notes::Hand)
    }

    fn has_jump_notes(&self) -> bool {
        self.notes.iter().any(|n| n.get_pattern() == Notes::Jump)
    }

    fn has_single_notes(&self) -> bool {
        self.notes.iter().any(|n| n.get_pattern() == Notes::Single)
    }

    fn determine_jack_type(&mut self) -> crate::mania::models::pattern::JackPattern {
        crate::mania::models::pattern::JackPattern::determine_jack_type(self)
    }

    fn determine_handstream_type(&mut self) -> crate::mania::models::pattern::HandstreamPattern {
        crate::mania::models::pattern::HandstreamPattern::determine_hs_type(self)
    }

    fn determine_jumpstream_type(&mut self) -> crate::mania::models::pattern::JumpstreamPattern {
        crate::mania::models::pattern::JumpstreamPattern::determine_js_type(self)
    }


    pub fn print_notes(&self) {
        for note in &self.notes {
            let line = note.to_display_string();
            println!("{}", line);
        }
    }

    pub fn t_notes(&self) -> i32 {
        self.notes
            .iter()
            .flat_map(|v| v.notes.iter())
            .filter(|&&b| b)
            .count() as i32
    }
}