use crate::text_buffer::{ArrayBuffer, TextMotions};

impl TextMotions for ArrayBuffer {
    fn prev(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        }
    }

    fn next(&mut self) {
        let line = self.get_line();
        let max_cols = line.end - line.start;

        if self.cursor.col < max_cols {
            self.cursor.col += 1;
        }
    }

    fn prev_line(&mut self) {
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
        }
    }

    fn next_line(&mut self) {
        let max_lines = self.text.matches('\n').count() + 1;
        if self.cursor.line < max_lines - 1 {
            self.cursor.line += 1
        }
    }

    fn start_line(&mut self) {
        self.cursor.col = 0;
    }

    fn end_line(&mut self) {
        let line = self.get_line();
        self.cursor.col = line.end - line.start;
    }
}
