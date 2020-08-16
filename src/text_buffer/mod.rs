pub mod array_buffer;
pub mod piece_table_buffer;

pub use array_buffer::ArrayBuffer;
pub use piece_table_buffer::PieceTableBuffer;

use std::ops::RangeBounds;

#[derive(Debug, Clone)]
pub struct Cursor {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Line {
    pub start: usize,
    pub end: usize,
}

pub trait TextOps {
    fn insert(&mut self, c: char);

    fn delete(&mut self);

    fn delete_range<R: RangeBounds<usize>>(&mut self, range: R);
}

pub trait TextMotions {
    fn next(&mut self);

    fn prev(&mut self);

    fn next_line(&mut self);

    fn prev_line(&mut self);

    fn start_line(&mut self);

    fn end_line(&mut self);
}

pub trait TextBuffer = TextOps + TextMotions;
