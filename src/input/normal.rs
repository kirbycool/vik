use crate::editor::{Editor, Mode};
use crate::text_buffer::TextBuffer;
use termion::cursor;
use termion::event::Key;

pub fn handle_normal_input(key: Key, editor: &mut Editor) {
    let cursor = editor.text_buffer.get_cursor();
    match key {
        Key::Char(':') => {
            editor.mode = Mode::Command;
        }
        Key::Char('i') => {
            editor.mode = Mode::Insert;
        }
        Key::Char('a') => {
            editor.text_buffer.set_cursor(cursor + 1);
            editor.mode = Mode::Insert;
        }
        Key::Left | Key::Char('h') => {
            editor
                .text_buffer
                .set_cursor(if cursor == 0 { 0 } else { cursor - 1 })
        }
        Key::Right | Key::Char('l') => editor.text_buffer.set_cursor(cursor + 1),
        _ => (),
    }
}
