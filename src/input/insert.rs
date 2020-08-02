use crate::editor::{Editor, Mode};
use crate::text_buffer::TextBuffer;
use termion::cursor;
use termion::event::Key;

pub fn handle_insert_input(key: Key, editor: &mut Editor) {
    let cursor = editor.text_buffer.get_cursor();
    match key {
        Key::Char(c) => {
            editor.text_buffer.insert(c.to_string().as_str());
        }
        Key::Esc => {
            print!("{}", cursor::SteadyBlock);
            editor.mode = Mode::Normal;
        }
        Key::Backspace => {
            if cursor == 0 {
                return;
            }

            editor.text_buffer.delete((cursor - 1)..cursor);
        }
        Key::Left => editor
            .text_buffer
            .set_cursor(if cursor == 0 { 0 } else { cursor - 1 }),
        Key::Right => editor.text_buffer.set_cursor(cursor + 1),
        _ => (),
    }
}
