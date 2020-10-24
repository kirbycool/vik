mod command;
mod delete_operator;
mod insert;
mod normal;

pub use command::CommandState;
pub use delete_operator::DeleteOperatorState;
pub use insert::InsertState;
pub use normal::NormalState;

use crate::editor::Editor;
use crate::event::Event;

pub enum State {
    Normal(NormalState),
    DeleteOperator(DeleteOperatorState),

    Insert(InsertState),

    Command(CommandState),
}

impl State {
    pub fn handle_event(self, event: Event, editor: &mut Editor) -> Vec<State> {
        match self {
            State::Normal(s) => s.handle_event(event, editor),
            State::Command(s) => s.handle_event(event, editor),
            State::Insert(s) => s.handle_event(event, editor),
            State::DeleteOperator(s) => s.handle_event(event, editor),
        }
    }
}
