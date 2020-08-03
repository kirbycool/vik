use crate::editor::{Editor, Mode};
use crate::text_buffer::TextBuffer;
use termion::event::Key;

pub fn handle_command_input(key: Key, editor: &mut Editor) {
    let cursor = editor.command_buffer.get_cursor();
    match key {
        Key::Char('\n') => {
            editor.run_command();
            editor.command_buffer.delete_range(..);
            editor.mode = Mode::Normal;
        }
        Key::Char(c) => {
            editor.command_buffer.insert(c);
        }
        Key::Esc => {
            editor.command_buffer.delete_range(..);
            editor.mode = Mode::Normal;
        }
        Key::Backspace => editor.command_buffer.delete(),
        Key::Left => editor.command_buffer.move_cursor_x(-1),
        Key::Right => editor.command_buffer.move_cursor_x(1),
        _ => (),
    }
}
