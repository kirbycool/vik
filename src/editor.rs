use crate::file::{load_file, write_file};
use crate::text_buffer::{ArrayBuffer, PieceTableBuffer};
use crate::ui::text_window::TextWindowState;
use std::error::Error;
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
    pub text_buffer: PieceTableBuffer,
    pub command_buffer: ArrayBuffer,
    pub mode: Mode,
    pub running: bool,
    pub filename: Option<String>,
    pub text_window_state: TextWindowState,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            text_buffer: PieceTableBuffer::new("".to_string()),
            command_buffer: ArrayBuffer::new("".to_string()),
            mode: Mode::Normal,
            running: true,
            filename: None,
            text_window_state: TextWindowState::new(),
        }
    }

    pub fn from_file(filename: String) -> Self {
        let mut editor = Editor::new();
        editor.text_buffer = load_file(filename.as_str()).unwrap();
        editor.filename = Some(filename);
        editor
    }

    pub fn run_command(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        let parts = self
            .command_buffer
            .text
            .split_whitespace()
            .collect::<Vec<&str>>();
        let (&command, args) = parts.split_first().ok_or("Invalid command")?;

        match command {
            "q" => self.running = false,
            "w" => match args {
                [] => {
                    if let Some(filename) = &self.filename {
                        write_file(filename.as_str(), &self.text_buffer)?;
                    }
                }
                [filename] => {
                    write_file(filename, &self.text_buffer)?;
                }
                _ => (),
            },
            "edit" | "e" => match args {
                [filename] => {
                    self.filename = Some(filename.to_string());
                    self.text_buffer = load_file(filename).unwrap();
                }
                _ => (),
            },
            _ => (),
        };
        Ok(())
    }
}
