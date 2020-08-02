pub mod array_buffer;

pub use array_buffer::ArrayBuffer;

use std::ops::RangeBounds;

pub trait TextBuffer {
    fn get_text<'a>(&'a self) -> &'a str;

    fn get_cursor(&self) -> usize;

    fn set_cursor(&mut self, pos: usize);

    fn insert(&mut self, text: &str);

    fn delete<R: RangeBounds<usize>>(&mut self, range: R);
}
