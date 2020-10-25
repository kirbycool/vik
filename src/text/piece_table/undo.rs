use super::{Node, PieceTableBuffer};

pub struct UndoStep {
    pieces: Vec<Node>,
    start: usize,
    end: usize,
}

impl UndoStep {
    pub fn new(start: usize, end: usize, pieces: Vec<Node>) -> Self {
        UndoStep { start, end, pieces }
    }
}

impl PieceTableBuffer {
    pub fn undo(&mut self) {
        let step = match self.undo_steps.pop() {
            Some(s) => s,
            None => return,
        };
        self.nodes.splice(step.start..step.end, step.pieces);
    }
}
