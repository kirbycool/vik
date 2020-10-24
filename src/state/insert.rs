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
        match key {
            Key::Char(c) => {
                editor.text_buffer.insert(c);
            }
            Key::Esc => {
                editor.text_buffer.prev();
                return vec![];
            }
            Key::Backspace => editor.text_buffer.delete(),
            Key::Left => editor.text_buffer.prev(),
            Key::Right => editor.text_buffer.next(),
            Key::Up => editor.text_buffer.prev_line(),
            Key::Down => editor.text_buffer.next_line(),
            _ => (),
        }
        vec![State::Insert(self)]
    }
}
