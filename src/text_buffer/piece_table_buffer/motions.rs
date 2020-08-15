use crate::text_buffer::{PieceTableBuffer, TextMotions};

impl TextMotions for PieceTableBuffer {
    fn prev(&mut self) {}

    fn next(&mut self) {}

    fn prev_line(&mut self) {}

    fn next_line(&mut self) {}

    fn start_line(&mut self) {}

    fn end_line(&mut self) {}
}
