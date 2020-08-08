use crate::editor::{Editor, Mode};
use crate::text_buffer::{TextBuffer, TextMotions};
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
        Key::Left => editor.text_buffer.prev(),
        Key::Right => editor.text_buffer.next(),
        Key::Up => editor.text_buffer.prev_line(),
        Key::Down => editor.text_buffer.next_line(),
        _ => (),
    }
}
