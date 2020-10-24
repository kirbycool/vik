use crate::buffer::Buffer;
use crate::event::Event;
use crate::file::load_file;
use crate::state::{NormalState, State};
use crate::text::PieceTableBuffer;
use crate::ui::text_window::TextWindowState;
use std::string::ToString;

pub struct Editor {
    pub text_buffer: Buffer<PieceTableBuffer>,
    pub state_stack: Vec<State>,
    pub running: bool,
    pub filename: Option<String>,
    pub text_window_state: TextWindowState,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            text_buffer: Buffer::new(Box::new(PieceTableBuffer::new("".to_string()))),
            state_stack: vec![State::Normal(NormalState::new())],
            running: true,
            filename: None,
            text_window_state: TextWindowState::new(),
        }
    }

    pub fn mode(&self) -> &str {
        match self.state() {
            State::Insert(_) => "INSERT",
            State::Command(_) => "COMMAND",

            State::Normal(_) => "NORMAL",
            State::DeleteOperator(_) => "DELETE",
        }
    }

    pub fn state(&self) -> &State {
        self.state_stack.last().unwrap()
    }

    pub fn from_file(filename: String) -> Self {
        let mut editor = Editor::new();
        editor.text_buffer = load_file(filename.as_str()).unwrap();
        editor.filename = Some(filename);
        editor
    }

    pub fn handle_event(&mut self, event: Event) {
        let state = self.state_stack.pop().unwrap();
        let mut new_states = state.handle_event(event, self);
        self.state_stack.append(&mut new_states)
    }
}
