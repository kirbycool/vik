use super::State;
use crate::editor::Editor;
use crate::event::Event;
use crate::text_object::TextObject;
use termion::event::Key;

#[derive(Clone)]
pub struct DeleteOperatorState {}

impl DeleteOperatorState {
    pub fn new() -> Self {
        DeleteOperatorState {}
    }

    pub fn handle_event(self, event: Event, editor: &mut Editor) -> Vec<State> {
        match event {
            Event::Key(key) => self.handle_key(key, editor),
        }
    }

    fn handle_key(self, key: Key, editor: &mut Editor) -> Vec<State> {
        match key {
            Key::Char('d') => {
                let line = editor.text_buffer.cursor.line;
                let text_object = TextObject::linewise(line, line);
            }
        }
        vec![]
    }
}
