use super::State;
use crate::editor::Editor;
use crate::event::Event;
use termion::event::Key;

#[derive(Clone, Debug)]
pub struct InsertState {}

impl InsertState {
    pub fn new() -> Self {
        InsertState {}
    }

    pub fn handle_event(self, event: Event, editor: &mut Editor) -> Vec<State> {
        match event {
            Event::Key(key) => self.handle_key(key, editor),
        }
    }

    fn handle_key(self, key: Key, editor: &mut Editor) -> Vec<State> {
        let buffer = &mut editor.text_buffer;
        match key {
            Key::Char(c) => {
                buffer.insert(c);
            }
            Key::Esc => {
                buffer.prev();
                buffer.text_buffer.cache_idx = None;
                return vec![];
            }
            Key::Backspace => buffer.delete(),
            Key::Left => buffer.move_cursor(buffer.prev()),
            Key::Right => buffer.move_cursor(buffer.next()),
            Key::Up => buffer.move_cursor(buffer.prev_line()),
            Key::Down => buffer.move_cursor(buffer.next_line()),
            _ => (),
        }
        vec![State::Insert(self)]
    }
}
