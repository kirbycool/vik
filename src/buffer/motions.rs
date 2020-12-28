use super::Buffer;
use super::Position;
use crate::text::TextBuffer;

const WORD_DELIMITERS: [char; 2] = [' ', '\t'];

impl<T: TextBuffer> Buffer<T> {
    pub fn prev(&self) -> Position {
        let col = if self.cursor.col > 0 {
            self.cursor.col - 1
        } else {
            self.cursor.col
        };
        Position::new(self.cursor.line, col)
    }

    pub fn next(&self) -> Position {
        let max_cols = self.text_buffer.line_length(self.cursor.line);
        let col = if self.cursor.col < max_cols {
            self.cursor.col + 1
        } else {
            self.cursor.col
        };
        Position::new(self.cursor.line, col)
    }

    pub fn prev_line(&self) -> Position {
        let line = if self.cursor.line > 0 {
            self.cursor.line - 1
        } else {
            self.cursor.line
        };
        Position::new(line, self.cursor.col)
    }

    pub fn next_line(&self) -> Position {
        let max_lines = self.text_buffer.line_count();
        let line = if self.cursor.line < max_lines - 1 {
            self.cursor.line + 1
        } else {
            self.cursor.line
        };
        Position::new(line, self.cursor.col)
    }

    pub fn start_line(&self) -> Position {
        Position::new(self.cursor.line, 0)
    }

    pub fn end_line(&self) -> Position {
        Position::new(
            self.cursor.line,
            self.text_buffer.line_length(self.cursor.line),
        )
    }
}
