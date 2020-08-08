use crate::editor::{Editor, Mode};
use crate::text_buffer::TextBuffer;
use std::io;
use termion::cursor;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Block, Paragraph};
use tui::{Frame, Terminal};

pub fn draw<B: Backend>(editor: &Editor, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let area = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(10),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(area);

        // There's some bug with the first line, so skip it for now
        draw_top_bar(editor, chunks[0], f);
        draw_text(editor, chunks[1], f);
        draw_statusline(editor, chunks[2], f);
        draw_commandline(editor, chunks[3], f);
    })
}

fn draw_top_bar<B: Backend>(editor: &Editor, area: Rect, frame: &mut Frame<B>) {
    let block = Block::default().style(Style::default().bg(Color::Gray).fg(Color::Black));
    frame.render_widget(block, area);
}

fn draw_text<B: Backend>(editor: &Editor, area: Rect, frame: &mut Frame<B>) {
    let text = Text::from(editor.text_buffer.get_text());
    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White).bg(Color::Black));
    frame.render_widget(paragraph, area);

    if editor.mode == Mode::Normal || editor.mode == Mode::Insert {
        if editor.mode == Mode::Normal {
            print!("{}", cursor::SteadyBlock);
        }
        if editor.mode == Mode::Insert {
            print!("{}", cursor::SteadyBar);
        }

        // Handle cursor
        let cursor = &editor.text_buffer.get_cursor();
        frame.set_cursor(
            area.x + cursor.col as u16 % area.width,
            area.y + cursor.line as u16 + cursor.col as u16 / area.width,
        )
    }
}

fn draw_statusline<B: Backend>(editor: &Editor, area: Rect, frame: &mut Frame<B>) {
    let status = format!(
        "{} cursor: {:?}, line: {:?}",
        editor.mode.to_string(),
        editor.text_buffer.get_cursor(),
        editor.text_buffer.get_line()
    );
    let text = Text::from(status.as_str());
    let paragraph = Paragraph::new(text).style(Style::default().bg(Color::Gray).fg(Color::Black));
    frame.render_widget(paragraph, area);
}

fn draw_commandline<B: Backend>(editor: &Editor, area: Rect, frame: &mut Frame<B>) {
    let style = Style::default().bg(Color::Black).fg(Color::White);
    if editor.mode != Mode::Command {
        let block = Block::default().style(style);
        frame.render_widget(block, area);
        return;
    }

    let command = format!(":{}", editor.command_buffer.get_text());
    let text = Text::from(command.as_str());
    let paragraph = Paragraph::new(text).style(style);
    frame.render_widget(paragraph, area);

    // Handle cursor
    let cursor = &editor.command_buffer.get_cursor();
    print!("{}", cursor::SteadyBlock);
    frame.set_cursor(
        area.x + cursor.col as u16 + 1,
        area.y, // Always one line
    )
}
