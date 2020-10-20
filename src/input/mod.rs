mod insert;
mod normal;

use crate::editor::{Editor, Mode};
use insert::handle_insert_input;
use normal::handle_normal_input;
use termion::event::Key;

pub fn handle_input(key: Key, editor: &mut Editor) {
    match editor.mode {
        Mode::Normal => handle_normal_input(key, editor),
        Mode::Insert => handle_insert_input(key, editor),
        _ => (),
    }
}
