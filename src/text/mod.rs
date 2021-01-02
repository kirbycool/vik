mod array;
mod piece_table;

pub use array::ArrayBuffer;
pub use piece_table::PieceTableBuffer;

use crate::buffer::Position;
use tui::text::Text;

#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub start: Position,
    pub length: usize,
}

impl Range {
    pub fn new(start: Position, length: usize) -> Self {
        Range { start, length }
    }
}

impl From<Position> for Range {
    fn from(pos: Position) -> Self {
        Range {
            start: pos,
            length: 1,
        }
    }
}

pub trait TextBuffer {
    type Iter<'a>: Iterator<Item = char>;

    fn to_string(&self) -> String;

    fn to_text(&self, start: usize, count: usize) -> Text;

    fn insert(&mut self, pos: Position, c: char);

    fn delete<T: Into<Range>>(&mut self, range: T);

    fn chars<'a>(&'a self, pos: Position) -> Self::Iter<'a>;

    fn line_count(&self) -> usize {
        self.chars(Position::new(0, 0))
            .filter(|&c| c == '\n')
            .count()
    }

    fn line(&self, line: usize) -> String {
        self.chars(Position::new(line, 0))
            .take_while(|&c| c != '\n')
            .collect::<String>()
    }

    fn line_length(&self, line: usize) -> usize {
        self.line(line).len()
    }
}
