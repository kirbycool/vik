use crate::editor::{Editor, Mode};
use crate::text_buffer::TextBuffer;
use termion::cursor;
use termion::event::Key;

pub fn handle_insert_input(key: Key, editor: &mut Editor) {
    match key {
        Key::Char(c) => {
            editor.text_buffer.insert(c);
        }
        Key::Esc => {
            print!("{}", cursor::SteadyBlock);
            editor.mode = Mode::Normal;
        }
        Key::Backspace => editor.text_buffer.delete(),
        Key::Left => editor.text_buffer.move_cursor_x(-1),
        Key::Right => editor.text_buffer.move_cursor_x(1),
        Key::Up => editor.text_buffer.move_cursor_y(-1),
        Key::Down => editor.text_buffer.move_cursor_y(1),
        _ => (),
    }
}
