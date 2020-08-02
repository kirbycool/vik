mod command;

use crate::editor::{Editor, Mode};
use command::handle_command_input;
use termion::event::Key;

pub fn handle_input(key: Key, editor: &mut Editor) {
    match editor.mode {
        Mode::Normal => match key {
            Key::Char(':') => editor.mode = Mode::Command,
            Key::Char('i') => editor.mode = Mode::Insert,
            _ => (),
        },
        Mode::Command => handle_command_input(key, editor),
        Mode::Insert => match key {
            Key::Esc => editor.mode = Mode::Normal,
            Key::Char(c) => {
                editor.text_buffer.push(c);
            }
            Key::Backspace => {
                editor.text_buffer.pop();
            }
            _ => (),
        },
    }
}
