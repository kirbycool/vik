use clap::{App, Arg};
use std::io;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::Terminal;
use vik::editor::Editor;
use vik::input::handle_input;
use vik::ui::draw;

fn main() {
    let clap_app = App::new("Vik").arg(
        Arg::with_name("FILE")
            .index(1)
            .required(false)
            .help("The file to open"),
    );
    let matches = clap_app.get_matches();

    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut keys = io::stdin().keys();

    let mut editor = match matches.value_of("FILE") {
        Some(filename) => Editor::from_file(filename.to_string()),
        None => Editor::new(),
    };

    // Initial draw
    draw(&mut editor, &mut terminal).unwrap();

    while editor.running {
        let mut should_draw = false;

        if let Some(Ok(key)) = keys.next() {
            handle_input(key, &mut editor);
            should_draw = true;
        }

        if should_draw {
            draw(&mut editor, &mut terminal).unwrap();
        }
    }
}
