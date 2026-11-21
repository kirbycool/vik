use clap::{App, Arg};
use log::debug;
use std::io;
use ratatui::{DefaultTerminal, crossterm};
use vik::editor::Editor;
use vik::event::Event;
use vik::logger::init_logger;
use vik::ui::draw;

// bye

fn main() {
    init_logger();

    ratatui::run(app);

}

fn app(terminal: &mut DefaultTerminal) {
    let clap_app = App::new("Vik").arg(
        Arg::with_name("FILE")
            .index(1)
            .required(false)
            .help("The file to open"),
    );
    let matches = clap_app.get_matches();

    let mut editor = match matches.value_of("FILE") {
        Some(filename) => Editor::from_file(filename.to_string()),
        None => Editor::new(),
    };

    debug!("Initialized");

    draw(&mut editor, terminal);

    while editor.running {
        let mut should_draw = false;

        match crossterm::event::read() {
            Ok(crossterm::event::Event::Key(event)) => {
                editor.handle_event(Event::Key(event));
                should_draw = true;
            },
            _ => continue
        }

        if should_draw {
            draw(&mut editor, terminal).unwrap();
        }
    }
}
