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
        match key {
            // Motions
            Key::Left | Key::Char('h') => editor.text_buffer.prev(),
            Key::Right | Key::Char('l') => editor.text_buffer.next(),
            Key::Up | Key::Char('k') => editor.text_buffer.prev_line(),
            Key::Down | Key::Char('j') => editor.text_buffer.next_line(),
            Key::Char('0') => editor.text_buffer.start_line(),
            Key::Char('$') => editor.text_buffer.end_line(),

            // Insert mode commands
            Key::Char('i') => return self.push_insert(),
            Key::Char('a') => {
                editor.text_buffer.next();
                return self.push_insert();
            }
            Key::Char('o') => {
                editor.text_buffer.line_below();
                return self.push_insert();
            }
            Key::Char('O') => {
                editor.text_buffer.line_above();
                return self.push_insert();
            }

            // Command mode commands
            Key::Char(':') => return self.push_command(),

            // Change commands
            Key::Char('x') => editor.text_buffer.delete(),

            // Operators
            Key::Char('d') => {
                return self.push_state(State::DeleteOperator(DeleteOperatorState::new()))
            }
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
