pub mod motions;

use crate::text_buffer::{Cursor, Line, TextOps};
use std::ops::Bound;
use std::ops::RangeBounds;
use tui::text::Text;

pub struct ArrayBuffer {
    pub text: String,
    pub cursor: Cursor,
}

impl ArrayBuffer {
    pub fn new(text: String) -> Self {
        ArrayBuffer {
            text,
            cursor: Cursor { line: 0, col: 0 },
        }
    }

    // Convert line + col to an index
    fn cursor_pos(&self) -> usize {
        let cursor = self.get_cursor();
        let line = self.get_line();
        if line.start + cursor.col > line.end {
            line.end
        } else {
            line.start + cursor.col
        }
    }

    // Update the cursor given a pos
    fn update_cursor(&mut self, pos: usize) {
        self.cursor.line = self.text[..pos].matches('\n').count();
        self.cursor.col = pos - self.get_line().start;
    }
}

impl TextOps for ArrayBuffer {
    fn get_contents<'a>(&'a self) -> &'a str {
        self.text.as_str()
    }

    fn get_text<'a>(&'a self) -> Text {
        Text::from(self.text.as_str())
    }

    fn get_line<'a>(&'a self) -> Line<'a> {
        let mut start = 0;
        for _ in 0..self.cursor.line {
            start = self.text[start..]
                .find('\n')
                .map_or(start, |i| start + i + 1)
        }

        let end = self.text[start..]
            .find('\n')
            .map_or(self.text.len(), |i| i + start);

        Line {
            text: &self.text[start..end],
            start,
            end,
        }
    }

    fn get_cursor(&self) -> Cursor {
        let line = self.get_line();
        let max_col = line.end - line.start;
        Cursor {
            line: self.cursor.line,
            col: if self.cursor.col > max_col {
                max_col
            } else {
                self.cursor.col
            },
        }
    }

    fn insert(&mut self, c: char) {
        let pos = self.cursor_pos();
        self.text.insert(pos, c);
        self.update_cursor(pos + 1);
    }

    fn delete(&mut self) {
        let pos = self.cursor_pos();
        if pos == 0 {
            return;
        }

        self.text.remove(pos - 1);
        self.update_cursor(pos - 1);
    }

    fn delete_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let new_pos = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
        };
        self.text.replace_range(range, "");
        if new_pos > self.text.len() {
            self.update_cursor(self.text.len())
        } else {
            self.update_cursor(new_pos);
        };
    }
}
