pub mod motions;

use crate::text_buffer::{Cursor, Line, TextBuffer};
use std::ops::Bound;
use std::ops::RangeBounds;

pub struct ArrayBuffer {
    text: String,
    cursor: usize,
}

impl ArrayBuffer {
    pub fn new(text: String) -> Self {
        ArrayBuffer { text, cursor: 0 }
    }
}

impl TextBuffer for ArrayBuffer {
    fn get_text<'a>(&'a self) -> &'a str {
        self.text.as_str()
    }

    fn get_line<'a>(&'a self) -> Line<'a> {
        let start = self.text[..self.cursor].rfind('\n').map_or(0, |i| i + 1);
        let end = self.text[self.cursor..]
            .find('\n')
            .map_or(self.text.len() - 1, |i| self.cursor + i);

        Line {
            text: &self.text[start..end],
            start,
            end,
        }
    }

    fn get_cursor(&self) -> Cursor {
        Cursor {
            line: self.text[..self.cursor].matches('\n').count(),
            col: self.cursor - self.get_line().start,
        }
    }

    fn insert(&mut self, c: char) {
        self.text.insert(self.cursor, c);
        self.cursor += 1;
    }

    fn delete(&mut self) {
        if self.cursor == 0 {
            return;
        }

        self.text.remove(self.cursor - 1);
        self.cursor -= 1
    }

    fn delete_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let new_cursor = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
        };
        self.text.replace_range(range, "");
        self.cursor = if new_cursor > self.text.len() - 1 {
            self.text.len() - 1
        } else {
            new_cursor
        };
    }
}
