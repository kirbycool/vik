use crate::text_buffer::{Cursor, Line, TextBuffer};
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

    fn get_line<'a>(&'a self) -> Line<'a> {
        let start = self.text[..self.cursor].rfind('\n').map_or(0, |i| i + 1);
        let end = self.text[self.cursor..]
            .find('\n')
            .map_or(self.text.len(), |i| self.cursor + i);

        Line {
            text: &self.text[start..end],
            start,
            end,
        }
    }

    fn get_cursor(&self) -> Cursor {
        Cursor {
            pos: self.cursor,
            line: self.text[..self.cursor].matches('\n').count(),
            col: self.cursor - self.get_line().start,
        }
    }

    fn set_cursor(&mut self, pos: usize) {
        self.cursor = if pos > self.text.len() {
            self.text.len()
        } else {
            pos
        }
    }

    fn move_cursor_x(&mut self, offset: isize) {
        let new_cursor = self.cursor as isize + offset;
        if new_cursor < 0 {
            self.cursor = 0;
        } else if new_cursor > self.text.len() as isize {
            self.cursor = self.text.len();
        } else {
            self.cursor = new_cursor as usize;
        }
    }

    fn move_cursor_y(&mut self, offset: isize) {
        if offset < 0 {
            let line = self.get_line();
            if line.start == 0 {
                return;
            }

            // Move to the previous line and position the cursor
            let col = self.cursor - line.start;
            self.cursor = line.start - 1;

            let line = self.get_line();
            let pos = line.start + col;

            self.cursor = if pos > line.end { line.end } else { pos }
        }

        if offset > 0 {
            let line = self.get_line();
            if line.end == self.text.len() {
                return;
            }

            // Move to the next line and position the cursor
            let col = self.cursor - line.start;
            self.cursor = line.end + 1;

            let line = self.get_line();
            let pos = line.start + col;

            self.cursor = if pos > line.end { line.end } else { pos }
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
        self.cursor = if new_cursor > self.text.len() {
            self.text.len()
        } else {
            new_cursor
        };
    }
}
