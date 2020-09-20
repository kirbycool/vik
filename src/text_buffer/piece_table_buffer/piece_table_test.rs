use super::*;
use std::rc::Rc;

#[test]
fn test_init() {
    let text = "foo bar\nbaz".to_string();
    let table = PieceTable::new(text.clone());
    assert_eq!(table.text(), text)
}

fn insert_seq(mut line: usize, mut col: usize, text: &str, table: &mut PieceTable) {
    for c in text.chars() {
        table.insert(table.cursor_location(line, col), c);
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
    let mut table = PieceTable::new("the jumps".to_string());

    insert_seq(0, 4, "quick fox ", &mut table);
    assert_eq!(table.text(), "the quick fox jumps".to_string());
    assert_eq!(table.nodes.len(), 3);

    insert_seq(0, 9, "\nbrown", &mut table);
    assert_eq!(table.text(), "the quick\nbrown fox jumps".to_string());
    assert_eq!(table.nodes.len(), 5);

    insert_seq(1, 15, "\nover dog", &mut table);
    assert_eq!(
        table.text(),
        "the quick\nbrown fox jumps\nover dog".to_string()
    );
    assert_eq!(table.nodes.len(), 6);

    insert_seq(2, 5, "the lazy ", &mut table);
    assert_eq!(
        table.text(),
        "the quick\nbrown fox jumps\nover the lazy dog".to_string()
    );
    assert_eq!(table.nodes.len(), 8);
}

#[test]
fn test_delete() {
    let original = Rc::new("the quick\nbrown".to_string());
    let added = Rc::new(" fox\njumps".to_string());
    let mut table = PieceTable {
        original: original.clone(),
        added: added.clone(),
        nodes: vec![
            Node::new(original.clone(), 0, 5),
            Node::new(original.clone(), 5, 10),
            Node::new(added.clone(), 0, 5),
            Node::new(added.clone(), 5, 5),
        ],
    };

    // From front
    table.delete(table.cursor_location(0, 0));
    table.delete(table.cursor_location(0, 0));
    table.delete(table.cursor_location(0, 0));
    table.delete(table.cursor_location(0, 0));
    table.delete(table.cursor_location(0, 0));
    println!("{:?}", table.nodes);
    assert_eq!(table.text(), "uick\nbrown fox\njumps".to_string());
    assert_eq!(table.nodes.len(), 3);

    // From the end
    table.delete(table.cursor_location(2, 4));
    table.delete(table.cursor_location(2, 3));
    table.delete(table.cursor_location(2, 2));
    table.delete(table.cursor_location(2, 1));
    table.delete(table.cursor_location(2, 0));
    table.delete(table.cursor_location(1, 9));
    println!("{:?}", table.nodes);
    assert_eq!(table.text(), "uick\nbrown fox".to_string());
    assert_eq!(table.nodes.len(), 2);

    // From the middle
    table.delete(table.cursor_location(1, 7));
    table.delete(table.cursor_location(1, 6));
    println!("{:?}", table.nodes);
    assert_eq!(table.text(), "uick\nbrown x".to_string());
    assert_eq!(table.nodes.len(), 3);
}

#[test]
fn test_cursor() {
    let original = Rc::new("the quick\nbrown".to_string());
    let added = Rc::new("jumped\nover the lazy dog".to_string());
    let table = PieceTable {
        original: original.clone(),
        added: added.clone(),
        nodes: vec![
            Node::new(original.clone(), 0, 5),
            Node::new(original.clone(), 5, 10),
            Node::new(added.clone(), 0, 10),
            Node::new(added.clone(), 10, 14),
        ],
    };

    assert_eq!(table.cursor_location(0, 0), Location { idx: 0, offset: 0 });
    assert_eq!(table.cursor_location(0, 3), Location { idx: 0, offset: 3 });
    assert_eq!(table.cursor_location(0, 5), Location { idx: 1, offset: 0 });
    assert_eq!(table.cursor_location(0, 7), Location { idx: 1, offset: 2 });
    assert_eq!(table.cursor_location(1, 2), Location { idx: 1, offset: 7 });
    assert_eq!(table.cursor_location(1, 5), Location { idx: 2, offset: 0 });
    assert_eq!(table.cursor_location(1, 7), Location { idx: 2, offset: 2 });
    assert_eq!(table.cursor_location(2, 1), Location { idx: 2, offset: 8 });
    assert_eq!(table.cursor_location(2, 10), Location { idx: 3, offset: 7 });

    // Past the line end
    assert_eq!(table.cursor_location(0, 12), Location { idx: 1, offset: 4 });
    assert_eq!(table.cursor_location(1, 15), Location { idx: 2, offset: 6 });
    assert_eq!(table.cursor_location(3, 20), Location { idx: 4, offset: 0 });
}

#[test]
fn test_line_length() {
    let original = Rc::new("the quick\nbrown".to_string());
    let added = Rc::new("jumped\nover the lazy dog".to_string());
    let table = PieceTable {
        original: original.clone(),
        added: added.clone(),
        nodes: vec![
            Node::new(original.clone(), 0, 5),
            Node::new(original.clone(), 5, 10),
            Node::new(added.clone(), 0, 10),
            Node::new(added.clone(), 10, 14),
        ],
    };

    assert_eq!(table.line_length(0), 9);
    assert_eq!(table.line_length(1), 11);
    assert_eq!(table.line_length(2), 17);
    assert_eq!(table.line_length(3), 0);
}
