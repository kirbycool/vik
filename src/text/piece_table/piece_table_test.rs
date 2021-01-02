use super::*;
use crate::buffer::Position;
use crate::text::Range;
use std::rc::Rc;

fn pos(line: usize, col: usize) -> Position {
    Position { line, col }
}

#[test]
fn test_init() {
    let text = "foo bar\nbaz".to_string();
    let table = PieceTableBuffer::new(text.clone());
    assert_eq!(table.to_string(), text)
}

fn insert_seq(mut line: usize, mut col: usize, text: &str, table: &mut PieceTableBuffer) {
    for c in text.chars() {
        table.insert(pos(line, col), c);
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
}

#[test]
fn test_insert() {
    let mut table = PieceTableBuffer::new("the jumps".to_string());

    insert_seq(0, 4, "quick fox ", &mut table);
    assert_eq!(table.to_string(), "the quick fox jumps".to_string());
    assert_eq!(table.pieces.len(), 3);

    insert_seq(0, 9, "\nbrown", &mut table);
    assert_eq!(table.to_string(), "the quick\nbrown fox jumps".to_string());
    assert_eq!(table.pieces.len(), 5);

    insert_seq(1, 15, "\nover dog", &mut table);
    assert_eq!(
        table.to_string(),
        "the quick\nbrown fox jumps\nover dog".to_string()
    );
    assert_eq!(table.pieces.len(), 6);

    insert_seq(2, 5, "the lazy ", &mut table);
    assert_eq!(
        table.to_string(),
        "the quick\nbrown fox jumps\nover the lazy dog".to_string()
    );
    assert_eq!(table.pieces.len(), 8);
}

#[test]
fn test_delete() {
    let mut table = PieceTableBuffer::new("the quick\nbrown".to_string());
    table.added = Rc::new(" fox\njumps".to_string());
    table.pieces = vec![
        Piece::new(table.original.clone(), 0, 5),
        Piece::new(table.original.clone(), 5, 10),
        Piece::new(table.added.clone(), 0, 5),
        Piece::new(table.added.clone(), 5, 5),
    ];

    // From front
    table.delete(pos(0, 0));
    table.delete(pos(0, 0));
    table.delete(pos(0, 0));
    table.delete(pos(0, 0));
    table.delete(pos(0, 0));
    assert_eq!(table.to_string(), "uick\nbrown fox\njumps".to_string());
    assert_eq!(table.pieces.len(), 3);

    // From the end
    table.delete(pos(2, 4));
    table.delete(pos(2, 3));
    table.delete(pos(2, 2));
    table.delete(pos(2, 1));
    table.delete(pos(2, 0));
    table.delete(pos(1, 9));
    assert_eq!(table.to_string(), "uick\nbrown fox".to_string());
    assert_eq!(table.pieces.len(), 2);

    // From the middle
    table.delete(pos(1, 7));
    table.delete(pos(1, 6));
    assert_eq!(table.to_string(), "uick\nbrown x".to_string());
    assert_eq!(table.pieces.len(), 3);

    // Range
    table.delete(Range::new(pos(0, 2), 5));
    assert_eq!(table.to_string(), "uiown x".to_string());
    assert_eq!(table.pieces.len(), 4);
}

#[test]
fn test_line_length() {
    let mut table = PieceTableBuffer::new("the quick\nbrown".to_string());
    table.added = Rc::new("jumped\nover the lazy dog".to_string());
    table.pieces = vec![
        Piece::new(table.original.clone(), 0, 5),
        Piece::new(table.original.clone(), 5, 10),
        Piece::new(table.added.clone(), 0, 10),
        Piece::new(table.added.clone(), 10, 14),
    ];

    // assert_eq!(table.line_length(0), 9);
    // assert_eq!(table.line_length(1), 11);
    // assert_eq!(table.line_length(2), 17);
    assert_eq!(table.line_length(3), 0);
}
