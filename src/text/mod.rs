mod array;
mod piece_table;

use crate::buffer::Position;
pub use array::ArrayBuffer;
pub use piece_table::PieceTableBuffer;
use tui::text::Text;

pub trait TextBuffer {
    fn to_string(&self) -> String;

    fn to_text(&self, start: usize, count: usize) -> Text;

    fn insert(&mut self, pos: Position, c: char);

    fn delete(&mut self, pos: Position);

    fn line_length(&self, n: usize) -> usize;

    fn line_count(&self) -> usize;
}
