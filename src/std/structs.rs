use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct Notes {
    pub(crate) timestamp: i32,
    pub(crate) pattern: BasePattern,
    pub(crate) coordinates: Vec<f32>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BasePattern {
    SINGLE,
    SLIDERS,
}
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum SecondaryPattern
{
    JUMP,
    STREAM,
    None,
}
impl Serialize for SecondaryPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for SecondaryPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecondaryPattern::JUMP => write!(f, "JUMP"),
            SecondaryPattern::STREAM => write!(f, "STREAM"),
            SecondaryPattern::None => write!(f, "None"),
        }
    }
}

#[derive(Debug)]
pub struct StdMeasure {
    pub(crate) measure: crate::structs::CommonMeasure,
    pub(crate) notes: Vec<Notes>,
    pub(crate) secondary_pattern: SecondaryPattern,
}