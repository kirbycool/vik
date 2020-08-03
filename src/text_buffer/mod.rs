pub mod array_buffer;

pub use array_buffer::ArrayBuffer;

use std::ops::RangeBounds;

pub struct Cursor {
    pub pos: usize,
    pub line: usize,
    pub col: usize,
}

pub struct Line<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
}

pub trait TextBuffer {
    fn get_text<'a>(&'a self) -> &'a str;

    fn get_line<'a>(&'a self) -> Line<'a>;

    fn get_cursor(&self) -> Cursor;

    fn set_cursor(&mut self, pos: usize);

    fn move_cursor_x(&mut self, offset: isize);

    fn move_cursor_y(&mut self, offset: isize);

    fn insert(&mut self, c: char);

    fn delete(&mut self);

    fn delete_range<R: RangeBounds<usize>>(&mut self, range: R);
}
