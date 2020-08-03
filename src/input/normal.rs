use crate::editor::{Editor, Mode};
use crate::text_buffer::TextBuffer;
use termion::event::Key;

pub fn handle_normal_input(key: Key, editor: &mut Editor) {
    match key {
        Key::Char(':') => {
            editor.mode = Mode::Command;
        }
        Key::Char('i') => {
            editor.mode = Mode::Insert;
        }
        Key::Char('a') => {
            editor.text_buffer.move_cursor_x(1);
            editor.mode = Mode::Insert;
        }
        Key::Left | Key::Char('h') => editor.text_buffer.move_cursor_x(-1),
        Key::Right | Key::Char('l') => editor.text_buffer.move_cursor_x(1),
        Key::Up | Key::Char('k') => editor.text_buffer.move_cursor_y(-1),
        Key::Down | Key::Char('j') => editor.text_buffer.move_cursor_y(1),
        Key::Char('0') => editor
            .text_buffer
            .set_cursor(editor.text_buffer.get_line().start),
        Key::Char('$') => editor
            .text_buffer
            .set_cursor(editor.text_buffer.get_line().end),
        _ => (),
    }
}
