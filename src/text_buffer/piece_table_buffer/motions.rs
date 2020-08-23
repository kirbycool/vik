use crate::text_buffer::{PieceTableBuffer, TextMotions};

impl TextMotions for PieceTableBuffer {
    fn prev(&mut self) {
        let virtual_cursor = self.cursor();
        if virtual_cursor.col > 0 {
            self.cursor.col = virtual_cursor.col - 1;
        }
    }

    fn next(&mut self) {
        let end_col = self.line_length();
        if self.cursor.col < end_col {
            self.cursor.col += 1;
        }
    }

    fn prev_line(&mut self) {
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
        }
    }

    fn next_line(&mut self) {
        let max_lines = self.line_count();
        if self.cursor.line < max_lines - 1 {
            self.cursor.line += 1
        }
    }

    fn start_line(&mut self) {
        self.cursor.col = 0;
    }

    fn end_line(&mut self) {
        self.cursor.col = self.line_length();
    }
}
