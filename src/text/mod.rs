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
    fn to_string(&self) -> String;

    fn to_text(&self, start: usize, count: usize) -> Text;

    fn insert(&mut self, pos: Position, c: char);

    fn delete<T: Into<Range>>(&mut self, range: T);

    fn line(&self, line: usize) -> String;

    fn line_length(&self, line: usize) -> usize;

    fn line_count(&self) -> usize;
}
