use crate::input::handle_input;
use crate::text_buffer::{ArrayBuffer, TextBuffer};
use crate::ui::draw;
use std::fmt;
use std::io;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::Terminal;

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
    pub text_buffer: String,
    pub command_buffer: ArrayBuffer,
    pub mode: Mode,
    pub running: bool,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            text_buffer: "".to_string(),
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

// The main run loop
pub fn run_loop() {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut keys = io::stdin().keys();

    let mut editor = Editor::new();

    // Initial draw
    draw(&editor, &mut terminal).unwrap();

    while editor.running {
        let mut should_draw = false;
        let editor = &mut editor;

        if let Some(Ok(key)) = keys.next() {
            handle_input(key, editor);
            should_draw = true;
        }

        if should_draw {
            draw(&editor, &mut terminal).unwrap();
        }
    }
}
