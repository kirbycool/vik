use crate::text_buffer::TextBuffer;
use std::ops::Bound;
use std::ops::RangeBounds;

pub struct ArrayBuffer {
    text: String,
    cursor: usize,
}

impl ArrayBuffer {
    pub fn new(text: String) -> Self {
        let cursor = text.len();
        ArrayBuffer { text, cursor }
    }
}

impl TextBuffer for ArrayBuffer {
    fn get_text<'a>(&'a self) -> &'a str {
        self.text.as_str()
    }

    fn get_cursor(&self) -> usize {
        self.cursor
    }

    fn set_cursor(&mut self, pos: usize) {
        self.cursor = if pos > self.text.len() {
            self.text.len()
        } else {
            pos
        }
    }

    fn insert(&mut self, text: &str) {
        self.text.insert_str(self.cursor, text);
        self.cursor += text.len();
    }

    fn delete<R: RangeBounds<usize>>(&mut self, range: R) {
        let new_cursor = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
        };
        self.text.replace_range(range, "");
        self.cursor = if new_cursor > self.text.len() {
            self.text.len()
        } else {
            new_cursor
        };
    }
}
