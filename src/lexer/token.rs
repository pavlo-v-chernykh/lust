#[derive(Debug, PartialEq)]
pub struct Pos {
    line: usize,
    col: usize,
}

impl Pos {
    fn new(line: usize, col: usize) -> Pos {
        Pos {
            line: line,
            col: col,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Span {
    start: Pos,
    end: Pos,
}

impl Span {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Span {
        Span {
            start: Pos::new(start_line, start_col),
            end: Pos::new(end_line, end_col),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number {
        span: Span,
        val: f64
    },
    String {
        span: Span,
        val: String,
    },
    Symbol {
        span: Span,
        val: String,
    },
    ListStart {
        span: Span,
    },
    ListEnd {
        span: Span,
    },
}
