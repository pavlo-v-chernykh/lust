use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct LexerError {
    line: usize,
    col: usize,
}

impl LexerError {
    pub fn new(line: usize, col: usize) -> LexerError {
        LexerError {
            line: line,
            col: col,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid syntax at {}:{}", self.line, self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::LexerError;

    #[test]
    fn test_descriptions_for_error() {
        let err = LexerError::new(1, 10);
        assert_eq!("Invalid syntax at 1:10", format!("{}", err));
    }
}
