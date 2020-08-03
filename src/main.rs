use std::io;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::Terminal;
use vik::editor::Editor;
use vik::input::handle_input;
use vik::ui::draw;

fn main() {
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
