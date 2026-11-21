use ratatui::crossterm::event::{KeyCode, KeyEvent};

use super::{CommandState, DeleteOperatorState, InsertState, State};
use crate::editor::Editor;
use crate::event::Event;

#[derive(Clone, Debug)]
pub struct NormalState {}

impl NormalState {
    pub fn new() -> Self {
        NormalState {}
    }

    pub fn handle_event(self, event: Event, editor: &mut Editor) -> Vec<State> {
        match event {
            Event::Key(key_event) => self.handle_key(key_event, editor),
        }
    }

    fn handle_key(self, key_event: KeyEvent, editor: &mut Editor) -> Vec<State> {
        let buffer = &mut editor.text_buffer;
        match key_event.code {
            // Motions
            KeyCode::Left | KeyCode::Char('h') => buffer.move_cursor(buffer.prev()),
            KeyCode::Right | KeyCode::Char('l') => buffer.move_cursor(buffer.next()),
            KeyCode::Up | KeyCode::Char('k') => buffer.move_cursor(buffer.prev_line()),
            KeyCode::Down | KeyCode::Char('j') => buffer.move_cursor(buffer.next_line()),
            KeyCode::Char('0') => buffer.move_cursor(buffer.start_line()),
            KeyCode::Char('$') => buffer.move_cursor(buffer.end_line()),

            // Insert mode commands
            KeyCode::Char('i') => return self.push_insert(),
            KeyCode::Char('a') => {
                buffer.next();
                return self.push_insert();
            }
            KeyCode::Char('o') => {
                buffer.line_below();
                return self.push_insert();
            }
            KeyCode::Char('O') => {
                buffer.line_above();
                return self.push_insert();
            }

            // Command mode commands
            KeyCode::Char(':') => return self.push_command(),

            // Change commands
            KeyCode::Char('x') => buffer.delete(),

            // Operators
            KeyCode::Char('d') => {
                return self.push_state(State::DeleteOperator(DeleteOperatorState::new()))
            }

            // Undo/redo
            KeyCode::Char('u') => buffer.text_buffer.undo(),

            _ => (),
        }
        vec![State::Normal(self)]
    }

    fn push_state(self, state: State) -> Vec<State> {
        vec![State::Normal(self), state]
    }

    fn push_insert(self) -> Vec<State> {
        self.push_state(State::Insert(InsertState::new()))
    }

    fn push_command(self) -> Vec<State> {
        self.push_state(State::Command(CommandState::new()))
    }
}
