use super::{Piece, PieceTableBuffer};

#[derive(Debug)]
pub struct UndoStep {
    pub pieces: Vec<Piece>,
    pub start: usize,
    pub end: usize,
}

impl UndoStep {
    pub fn new(start: usize, end: usize, pieces: Vec<Piece>) -> Self {
        UndoStep { start, end, pieces }
    }
}

impl PieceTableBuffer {
    pub fn undo(&mut self) {
        let step = match self.undo_steps.pop() {
            Some(s) => s,
            None => return,
        };
        self.pieces.splice(step.start..step.end, step.pieces);
    }
}
