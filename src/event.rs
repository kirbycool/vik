use ratatui::crossterm::event;

pub enum Event {
    Key(event::KeyEvent),
}
