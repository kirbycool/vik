use super::{CommandState, InsertState, State};
use crate::editor::Editor;
use crate::event::Event;
use termion::event::Key;

#[derive(Clone)]
pub struct NormalState {}

impl NormalState {
    pub fn new() -> Self {
        NormalState {}
    }

    pub fn handle_event(self, event: Event, editor: &mut Editor) -> Vec<State> {
        match event {
            Event::Key(key) => self.handle_key(key, editor),
        }
    }

    fn handle_key(self, key: Key, editor: &mut Editor) -> Vec<State> {
        match key {
            // Change mode
            Key::Char(':') => {
                return vec![State::Normal(self), State::Command(CommandState::new())];
            }
            Key::Char('i') => {
                return vec![State::Normal(self), State::Insert(InsertState::new())];
            }
            Key::Char('a') => {
                editor.text_buffer.next();
                return vec![State::Normal(self), State::Insert(InsertState::new())];
            }

            // Motions
            Key::Left | Key::Char('h') => editor.text_buffer.prev(),
            Key::Right | Key::Char('l') => editor.text_buffer.next(),
            Key::Up | Key::Char('k') => editor.text_buffer.prev_line(),
            Key::Down | Key::Char('j') => editor.text_buffer.next_line(),
            Key::Char('0') => editor.text_buffer.start_line(),
            Key::Char('$') => editor.text_buffer.end_line(),

            Key::Char('x') => editor.text_buffer.delete(),
            _ => (),
        }
        vec![State::Normal(self)]
    }
}
