use super::piece_table::PieceTable;
use crate::text_buffer::{PieceTableBuffer, TextOps};
use std::ops;

impl TextOps for PieceTableBuffer {
    fn insert(&mut self, c: char) {
        // Update the cursor position
        if c == '\n' {
            self.cursor.col = 0;
            self.cursor.line = self.cursor.line + 1;
        } else {
            self.cursor.col = self.cursor().col + 1;
        }
    }

    fn delete(&mut self) {
        /*
        let cursor = self.cursor();
        if cursor.line == 0 && cursor.col == 0 {
            return;
        }

        // Update the cursor to the previous position
        if cursor.col > 0 {
            self.cursor.col -= 1
        } else {
            self.cursor.line -= 1;
            self.cursor.col = self.line_length();
        }
        debug!("cursor: {:?}, virtual: {:?}", self.cursor, self.cursor());

        let (node_index, offset) = self.cursor_node();
        let node = self.piece_table.nodes[node_index];

        // The node has length 1, so we can remove it
        if node.length == 0 {
            let piece_table = &mut self.piece_table;
            piece_table.nodes.remove(node_index);
            return;
        }

        // We're at the beginning of the node so we can just shrink it
        if offset == 0 {
            let new_node = self.piece_table.shrink_node_head(&node);
            let _ = mem::replace(&mut self.piece_table.nodes[node_index], new_node);
            return;
        }

        // We're at the end of the node, so we shrink it
        if offset == node.length - 1 {
            let new_node = self.piece_table.shrink_node_tail(&node);
            let _ = mem::replace(&mut self.piece_table.nodes[node_index], new_node);
            return;
        }

        // We have to split the node
        let (left, mut right) = self.piece_table.split_node(&node, offset);

        if right.length > 0 {
            right.length -= 1;
            right.start += 1;
        }

        let piece_table = &mut self.piece_table;
        piece_table.nodes.remove(node_index);
        if right.length > 0 {
            piece_table.nodes.insert(node_index, right);
        }
        if left.length > 0 {
            piece_table.nodes.insert(node_index, left);
        }
        debug!("{:?}", self.piece_table.nodes);
        */
    }

    fn delete_range<R: ops::RangeBounds<usize>>(&mut self, _range: R) {}
}
