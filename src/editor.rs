use std::fmt;
use std::io;
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::{Frame, Terminal};

enum Mode {
    Insert,
    Normal,
    Command,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Insert => write!(f, "Insert"),
            Mode::Normal => write!(f, "Normal"),
            Mode::Command => write!(f, "Command"),
        }
    }
}

struct Editor {
    pub text: String,
    pub mode: Mode,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            text: "abcdefg".to_string(),
            mode: Mode::Normal,
        }
    }
}

// The main run loop
pub fn run_loop() {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut stdin = async_stdin().keys();

    let mut editor = Editor::new();

    // Initial draw
    draw(&editor, &mut terminal).unwrap();

    loop {
        let mut should_draw = false;
        let editor = &mut editor;

        if let Some(Ok(key)) = stdin.next() {
            match editor.mode {
                Mode::Normal => match key {
                    Key::Char(':') => editor.mode = Mode::Command,
                    Key::Char('i') => editor.mode = Mode::Insert,
                    _ => (),
                },
                Mode::Command => match key {
                    Key::Char('q') => break,
                    _ => (),
                },
                Mode::Insert => match key {
                    Key::Esc => editor.mode = Mode::Normal,
                    Key::Char(c) => {
                        editor.text.push(c);
                    }
                    Key::Backspace => {
                        editor.text.pop();
                    }
                    _ => (),
                },
            }
            should_draw = true;
        }

        if should_draw {
            draw(&editor, &mut terminal).unwrap();
        }
    }
}

fn draw<B: Backend>(editor: &Editor, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let area = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(10),
                    Constraint::Length(2),
                ]
                .as_ref(),
            )
            .split(area);

        draw_text(editor, chunks[1], f);
        draw_statusline(editor, chunks[2], f);
    })
}

fn draw_text<B: Backend>(editor: &Editor, area: Rect, frame: &mut Frame<B>) {
    let text = Text::from(editor.text.as_str());
    let block = Block::default().style(Style::default().fg(Color::White).bg(Color::Black));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn draw_statusline<B: Backend>(editor: &Editor, area: Rect, frame: &mut Frame<B>) {
    let status = editor.mode.to_string();
    let text = Text::from(status.as_str());
    let block = Block::default().style(Style::default().bg(Color::Gray).fg(Color::Black));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
