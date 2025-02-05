#[derive(Debug, Clone)]
pub struct CommonNotes {
    timestamp: i32,
    notes: Vec<bool>,
}

#[derive(Debug)]
pub struct CommonMeasure {
    pub(crate) start_time: i32,
    pub(crate) npm: i32,
}

