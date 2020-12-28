use super::{CommandState, DeleteOperatorState, InsertState, State};
use crate::editor::Editor;
use crate::event::Event;
use termion::event::Key;

#[derive(Clone, Debug)]
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
        let buffer = &mut editor.text_buffer;
        match key {
            // Motions
            Key::Left | Key::Char('h') => buffer.move_cursor(buffer.prev()),
            Key::Right | Key::Char('l') => buffer.move_cursor(buffer.next()),
            Key::Up | Key::Char('k') => buffer.move_cursor(buffer.prev_line()),
            Key::Down | Key::Char('j') => buffer.move_cursor(buffer.next_line()),
            Key::Char('0') => buffer.move_cursor(buffer.start_line()),
            Key::Char('$') => buffer.move_cursor(buffer.end_line()),

            // Insert mode commands
            Key::Char('i') => return self.push_insert(),
            Key::Char('a') => {
                buffer.next();
                return self.push_insert();
            }
            Key::Char('o') => {
                buffer.line_below();
                return self.push_insert();
            }
            Key::Char('O') => {
                buffer.line_above();
                return self.push_insert();
            }

            // Command mode commands
            Key::Char(':') => return self.push_command(),

            // Change commands
            Key::Char('x') => buffer.delete(),

            // Operators
            Key::Char('d') => {
                return self.push_state(State::DeleteOperator(DeleteOperatorState::new()))
            }

            // Undo/redo
            Key::Char('u') => buffer.text_buffer.undo(),

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
