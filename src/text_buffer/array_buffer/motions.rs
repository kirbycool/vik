use crate::text_buffer::{ArrayBuffer, TextBuffer, TextMotions};

impl TextMotions for ArrayBuffer {
    fn prev(&mut self) {
        if self.cursor == 0 {
            return;
        }
        if self.text.as_bytes()[self.cursor - 1] as char == '\n' {
            return;
        }
        self.cursor = self.cursor - 1;
    }

    fn next(&mut self) {
        if self.cursor == self.text.len() - 1 {
            return;
        }
        if self.text.as_bytes()[self.cursor + 1] as char == '\n' {
            return;
        }
        self.cursor = self.cursor + 1;
    }

    fn prev_line(&mut self) {
        let line = self.get_line();
        if line.start == 0 {
            return;
        }

        let offset = self.cursor - line.start;

        // Move to the previous line and position the cursor
        self.cursor = line.start - 1;
        let line = self.get_line();
        let pos = line.start + offset;

        self.cursor = if pos > line.end { line.end } else { pos }
    }

    fn next_line(&mut self) {
        let line = self.get_line();
        if line.end == self.text.len() - 1 {
            return;
        }

        let offset = self.cursor - line.start;

        // Move to the next line and position the cursor
        self.cursor = line.end + 1;
        let line = self.get_line();
        let pos = line.start + offset;

        self.cursor = if pos > line.end { line.end } else { pos }
    }

    fn start_line(&mut self) {
        self.cursor = self.get_line().start
    }

    fn end_line(&mut self) {
        self.cursor = self.get_line().end
    }
}
