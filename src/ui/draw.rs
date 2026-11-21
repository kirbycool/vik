use crate::editor::Editor;
use crate::state::State;
use crate::text::TextBuffer;
use crate::ui::text_window::TextWindow;
use std::io;
use ratatui::backend::Backend;
use ratatui::crossterm::cursor::{SetCursorStyle};
use ratatui::crossterm::ExecutableCommand;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{Frame, Terminal};

pub fn draw<B: Backend>(editor: &mut Editor, terminal: &mut Terminal<B>) -> Result<(), B::Error> {
    terminal.draw(|f| {
        let area = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(10),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(area);

        // There's some bug with the first line, so skip it for now
        draw_text(editor, chunks[0], f);
        draw_statusline(editor, chunks[1], f);
        draw_commandline(editor, chunks[2], f);
    })?;

    Ok(())
}

fn draw_text(editor: &mut Editor, area: Rect, frame: &mut Frame) {
    let paragraph = TextWindow::new(&editor.text_buffer)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    frame.render_stateful_widget(paragraph, area, &mut editor.text_window_state);

    // Draw the cursor in the text
    let state = editor.state();

    let cursor_style = match state {
        State::Normal(_) => SetCursorStyle::SteadyBlock,
        State::Insert(_) => SetCursorStyle::SteadyBar,
        State::DeleteOperator(_) => SetCursorStyle::SteadyUnderScore,
        _ => return
    };
    io::stdout().execute(cursor_style).unwrap();

    let cursor = editor.text_buffer.cursor();
    let offset = editor.text_window_state.offset;
    frame.set_cursor(
        area.x + cursor.col as u16 % area.width,
        area.y + cursor.line as u16 + cursor.col as u16 / area.width - offset as u16,
    )
}

fn draw_statusline(editor: &Editor, area: Rect, frame: &mut Frame) {
    let status = format!(
        "{} | {}",
        editor.mode(),
        editor.filename.as_ref().unwrap_or(&"No File".to_string()),
    );
    let text = Text::from(status.as_str());
    let paragraph = Paragraph::new(text).style(Style::default().bg(Color::Gray).fg(Color::Black));
    frame.render_widget(paragraph, area);
}

fn draw_commandline(editor: &Editor, area: Rect, frame: &mut Frame) {
    let style = Style::default().bg(Color::Black).fg(Color::White);
    let state = match editor.state() {
        State::Command(s) => s,
        _ => {
            let block = Block::default().style(style);
            frame.render_widget(block, area);
            return;
        }
    };

    let text = format!(":{}", state.buffer.text_buffer.to_string());
    let paragraph = Paragraph::new(text.as_str()).style(style);
    frame.render_widget(paragraph, area);

    // Handle cursor
    let cursor = &state.buffer.cursor();
    print!("{}", SetCursorStyle::SteadyBlock);
    frame.set_cursor(
        area.x + cursor.col as u16 + 1,
        area.y, // Always one line
    )
}
