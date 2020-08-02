use crate::editor::{Editor, Mode};
use crate::text_buffer::TextBuffer;
use termion::event::Key;

pub fn handle_command_input(key: Key, editor: &mut Editor) {
    let cursor = editor.command_buffer.get_cursor();
    match key {
        Key::Char('\n') => {
            editor.run_command();
            editor.command_buffer.delete(..);
            editor.mode = Mode::Normal;
        }
        Key::Char(c) => {
            editor.command_buffer.insert(c.to_string().as_str());
        }
        Key::Esc => {
            editor.command_buffer.delete(..);
            editor.mode = Mode::Normal;
        }
        Key::Backspace => {
            if cursor == 0 {
                return;
            }

            editor.command_buffer.delete((cursor - 1)..cursor);
        }
        Key::Left => editor
            .command_buffer
            .set_cursor(if cursor == 0 { 0 } else { cursor - 1 }),
        Key::Right => editor.command_buffer.set_cursor(cursor + 1),
        _ => (),
    }
}
