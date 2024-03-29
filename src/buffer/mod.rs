pub mod motions;

use crate::text::TextBuffer;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Position { line, col }
    }
}

#[derive(Debug, Clone)]
pub struct Buffer<T: TextBuffer> {
    pub cursor: Position,
    pub text_buffer: Box<T>,
}

impl<T: TextBuffer> Buffer<T> {
    pub fn new(text_buffer: Box<T>) -> Self {
        Buffer {
            cursor: Position::new(0, 0),
            text_buffer,
        }
    }

    pub fn insert(&mut self, c: char) {
        let cursor = self.cursor();

        self.text_buffer.insert(self.cursor(), c);

        // Update the cursor position
        if c == '\n' {
            self.cursor.col = 0;
            self.cursor.line = cursor.line + 1;
        } else {
            self.cursor.col = cursor.col + 1;
        }
    }

    pub fn delete(&mut self) {
        let cursor = self.cursor();
        if cursor.line == 0 && cursor.col == 0 {
            return;
        }

        // Update the cursor to the previous position
        if cursor.col > 0 {
            self.cursor.col -= 1
        } else {
            self.cursor.line -= 1;
            self.cursor.col = self.text_buffer.line_length(self.cursor.line);
        }

        self.text_buffer.delete(self.cursor)
    }

    pub fn move_cursor(&mut self, pos: Position) {
        self.cursor = pos
    }

    pub fn cursor(&self) -> Position {
        let max_col = self.text_buffer.line_length(self.cursor.line);
        Position {
            line: self.cursor.line,
            col: if self.cursor.col > max_col {
                max_col
            } else {
                self.cursor.col
            },
        }
    }

    pub fn line_above(&mut self) {
        let line = self.cursor.line;
        self.text_buffer.insert(Position::new(line, 0), '\n');
        self.cursor = Position::new(line, 0);
    }

    pub fn line_below(&mut self) {
        let line = self.cursor.line;
        let col = self.text_buffer.line_length(line);
        self.text_buffer.insert(Position::new(line, col), '\n');
        self.cursor = Position::new(line + 1, 0);
    }
}
