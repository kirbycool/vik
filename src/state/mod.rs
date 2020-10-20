mod command;
mod normal;

pub use command::CommandState;
pub use normal::NormalState;

use crate::editor::Editor;
use crate::event::Event;

pub enum State {
    Command(CommandState),
    Normal(NormalState),
}

impl State {
    pub fn handle_event(self, event: Event, editor: &mut Editor) -> Vec<State> {
        match self {
            State::Normal(s) => s.handle_event(event, editor),
            State::Command(s) => s.handle_event(event, editor),
        }
    }
}
