#[derive(Debug, Clone)]
pub struct Notes {
    pub(crate) timestamp: i32,
    pub(crate) notes: Vec<bool>,
    pub(crate) pattern: BasePattern,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BasePattern {
    SINGLE,
    SLIDERS,
}