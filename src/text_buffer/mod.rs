pub mod array_buffer;

pub use array_buffer::ArrayBuffer;

use std::ops::RangeBounds;

#[derive(Debug)]
pub struct Cursor {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Line<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
}

pub trait TextBuffer {
    fn get_text<'a>(&'a self) -> &'a str;

    fn get_line<'a>(&'a self) -> Line<'a>;

    fn get_cursor(&self) -> Cursor;

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
