use termion;

pub enum Event {
    Key(termion::event::Key),
}
