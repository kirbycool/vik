use crate::editor::{Editor, Mode};
use crate::text_buffer::TextBuffer;
use termion::event::Key;

pub fn handle_normal_input(key: Key, editor: &mut Editor) {
    match key {
        // Change mode
        Key::Char(':') => {
            editor.mode = Mode::Command;
        }
        Key::Char('i') => {
            editor.mode = Mode::Insert;
        }
        Key::Char('a') => {
            editor.text_buffer.next();
            editor.mode = Mode::Insert;
        }

        // Motions
        Key::Left | Key::Char('h') => editor.text_buffer.prev(),
        Key::Right | Key::Char('l') => editor.text_buffer.next(),
        Key::Up | Key::Char('k') => editor.text_buffer.prev_line(),
        Key::Down | Key::Char('j') => editor.text_buffer.next_line(),
        Key::Char('0') => editor.text_buffer.start_line(),
        Key::Char('$') => editor.text_buffer.end_line(),

        Key::Char('x') => editor.text_buffer.delete(),
        _ => (),
    }
}
