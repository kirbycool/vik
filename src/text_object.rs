use crate::buffer::Position;

pub enum TextObject {
    Charwise(CharwiseObject),
    Linewise(LinewiseObject),
}

impl TextObject {
    pub fn charwise(start: Position, end: Position) -> Self {
        TextObject::Charwise(CharwiseObject { start, end })
    }

    pub fn linewise(start: usize, end: usize) -> Self {
        TextObject::Linewise(LinewiseObject { start, end })
    }
}

// Always inclusive
pub struct CharwiseObject {
    pub start: Position,
    pub end: Position,
}

// Always inclusive
pub struct LinewiseObject {
    pub start: usize,
    pub end: usize,
}
