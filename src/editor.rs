use crate::buffer::Buffer;
use crate::event::Event;
use crate::file::load_file;
use crate::state::{NormalState, State};
use crate::text::PieceTableBuffer;
use crate::ui::text_window::TextWindowState;
use std::cell::RefCell;
use std::fmt;
use std::string::ToString;

#[derive(PartialEq)]
pub enum Mode {
    Insert,
    Normal,
    Command,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Insert => write!(f, "INSERT"),
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Command => write!(f, "COMMAND"),
        }
    }
}

pub struct Editor {
    pub text_buffer: Buffer<PieceTableBuffer>,
    pub mode: Mode,
    pub state_stack: Vec<State>,
    pub running: bool,
    pub filename: Option<String>,
    pub text_window_state: TextWindowState,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            text_buffer: Buffer::new(Box::new(PieceTableBuffer::new("".to_string()))),
            mode: Mode::Normal,
            state_stack: vec![State::Normal(NormalState::new())],
            running: true,
            filename: None,
            text_window_state: TextWindowState::new(),
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
        let mut state = self.state_stack.pop().unwrap();
        let mut new_states = state.handle_event(event, self);
        self.state_stack.append(&mut new_states)
    }
}
