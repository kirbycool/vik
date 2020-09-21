use crate::editor::{Editor, Mode};
use termion::event::Key;

pub fn handle_command_input(key: Key, editor: &mut Editor) {
    match key {
        Key::Char('\n') => {
            editor.run_command().unwrap();
            // editor.command_buffer.clear();
            editor.mode = Mode::Normal;
        }
        Key::Char(c) => {
            editor.command_buffer.insert(c);
        }
        Key::Esc => {
            // editor.command_buffer.text_buffer.clear();
            editor.mode = Mode::Normal;
        }
        Key::Backspace => editor.command_buffer.delete(),
        Key::Left => editor.command_buffer.prev(),
        Key::Right => editor.command_buffer.next(),
        _ => (),
    }
}
