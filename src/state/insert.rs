use super::State;
use crate::editor::Editor;
use crate::event::Event;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

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

    fn handle_key(self, key: KeyEvent, editor: &mut Editor) -> Vec<State> {
        let buffer = &mut editor.text_buffer;
        match key.code {
            KeyCode::Char(c) => {
                buffer.insert(c);
            }
            KeyCode::Enter => buffer.insert('\n'),
            KeyCode::Tab => buffer.insert('\t'),
            KeyCode::Esc => {
                buffer.prev();
                buffer.text_buffer.cache_idx = None;
                return vec![];
            }
            KeyCode::Backspace => buffer.delete(),
            KeyCode::Left => buffer.move_cursor(buffer.prev()),
            KeyCode::Right => buffer.move_cursor(buffer.next()),
            KeyCode::Up => buffer.move_cursor(buffer.prev_line()),
            KeyCode::Down => buffer.move_cursor(buffer.next_line()),
            _ => (),
        }
        vec![State::Insert(self)]
    }
}
