use crate::text_buffer::{PieceTableBuffer, TextOps};
use std::ops;

impl TextOps for PieceTableBuffer {
    fn insert(&mut self, c: char) {
        let cursor = self.cursor();
        let location = self.piece_table.cursor_location(cursor.line, cursor.col);
        self.piece_table.insert(location, c);

        // Update the cursor position
        if c == '\n' {
            self.cursor.col = 0;
            self.cursor.line = self.cursor.line + 1;
        } else {
            self.cursor.col = self.cursor().col + 1;
        }
    }

    fn delete(&mut self) {
        let cursor = self.cursor();
        if cursor.line == 0 && cursor.col == 0 {
            return;
        }

        // Update the cursor to the previous position
        if cursor.col > 0 {
            self.cursor.col -= 1
        } else {
            self.cursor.line -= 1;
            self.cursor.col = self.line_length();
        }

        let location = self
            .piece_table
            .cursor_location(self.cursor.line, self.cursor.col);
        self.piece_table.delete(location)
    }

    fn delete_range<R: ops::RangeBounds<usize>>(&mut self, _range: R) {}
}
