use crate::text_buffer::{ArrayBuffer, TextBuffer};
use std::fmt;

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
    pub text_buffer: ArrayBuffer,
    pub command_buffer: ArrayBuffer,
    pub mode: Mode,
    pub running: bool,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            text_buffer: ArrayBuffer::new("line 1\nline2\nline3".to_string()),
            command_buffer: ArrayBuffer::new("".to_string()),
            mode: Mode::Normal,
            running: true,
        }
    }

    pub fn run_command(&mut self) {
        match self.command_buffer.get_text() {
            "q" => self.running = false,
            _ => (),
        }
    }
}
