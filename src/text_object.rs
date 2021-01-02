use crate::buffer::Position;
use crate::text::{Range, TextBuffer};

pub enum TextObject {
    Charwise(CharwiseObject),
    Linewise(LinewiseObject),
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

impl TextObject {
    pub fn charwise(start: Position, end: Position) -> Self {
        TextObject::Charwise(CharwiseObject { start, end })
    }

    pub fn linewise(start: usize, end: usize) -> Self {
        TextObject::Linewise(LinewiseObject { start, end })
    }

    pub fn range<'a, T: TextBuffer<'a>>(&self, text: &T) -> Range {
        use TextObject::*;

        match self {
            Charwise(_) => Range::new(Position::new(0, 0), 0),
            Linewise(obj) => {
                // TODO optimize this
                let length = (obj.start..=obj.end)
                    .map(|lineno| text.line_length(lineno) + 1) // include \n
                    .sum();
                Range::new(Position::new(obj.start, 0), length)
            }
        }
    }
}
