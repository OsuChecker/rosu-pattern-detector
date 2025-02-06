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
#[derive(Debug)]
pub struct StdMeasure {
    pub(crate) measure: crate::structs::CommonMeasure,
    pub(crate) notes: Vec<Notes>,
    pub(crate) secondary_pattern: SecondaryPattern,
}