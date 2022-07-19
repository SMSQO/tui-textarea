use tui_textarea::{CursorMove, TextArea};

const BOTTOM_RIGHT: CursorMove = CursorMove::Jump(u16::MAX, u16::MAX);

#[test]
fn test_empty_textarea() {
    use CursorMove::*;

    let mut t = TextArea::default();
    for m in [
        Forward,
        Back,
        Up,
        Down,
        Head,
        End,
        Top,
        Bottom,
        WordForward,
        WordBack,
        ParagraphForward,
        ParagraphBack,
        Jump(0, 0),
        Jump(u16::MAX, u16::MAX),
    ] {
        t.move_cursor(m);
        assert_eq!(t.cursor(), (0, 0), "{:?}", m);
    }
}

#[test]
fn test_forward() {
    let mut t = TextArea::from(["abc", "def"]);

    for pos in [
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 0),
        (1, 1),
        (1, 2),
        (1, 3),
        (1, 3),
    ] {
        t.move_cursor(CursorMove::Forward);
        assert_eq!(t.cursor(), pos);
    }
}

#[test]
fn test_back() {
    let mut t = TextArea::from(["abc", "def"]);
    t.move_cursor(BOTTOM_RIGHT);

    for pos in [
        (1, 2),
        (1, 1),
        (1, 0),
        (0, 3),
        (0, 2),
        (0, 1),
        (0, 0),
        (0, 0),
    ] {
        t.move_cursor(CursorMove::Back);
        assert_eq!(t.cursor(), pos);
    }
}

#[test]
fn test_up() {
    let mut t = TextArea::from(["abc", "def", "ghi"]);

    for col in 0..=3 {
        let mut row = 2;

        t.move_cursor(CursorMove::Jump(2 as u16, col as u16));
        assert_eq!(t.cursor(), (row, col));

        while row > 0 {
            t.move_cursor(CursorMove::Up);
            row -= 1;
            assert_eq!(t.cursor(), (row, col));
        }
    }
}

#[test]
fn test_up_trim() {
    let mut t = TextArea::from(["", "a", "bcd", "efgh"]);
    t.move_cursor(CursorMove::Jump(3, 3));

    for expected in [(2, 3), (1, 1), (0, 0)] {
        t.move_cursor(CursorMove::Up);
        assert_eq!(t.cursor(), expected);
    }
}

#[test]
fn test_down() {
    let mut t = TextArea::from(["abc", "def", "ghi"]);

    for col in 0..=3 {
        let mut row = 0;

        t.move_cursor(CursorMove::Jump(0, col as u16));
        assert_eq!(t.cursor(), (row, col));

        while row < 2 {
            t.move_cursor(CursorMove::Down);
            row += 1;
            assert_eq!(t.cursor(), (row, col));
        }
    }
}

#[test]
fn test_down_trim() {
    let mut t = TextArea::from(["abcd", "efg", "h", ""]);
    t.move_cursor(CursorMove::Jump(0, 3));

    for expected in [(1, 3), (2, 1), (3, 0)] {
        t.move_cursor(CursorMove::Down);
        assert_eq!(t.cursor(), expected);
    }
}
