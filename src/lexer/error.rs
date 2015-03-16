use std::fmt;

use self::LexerErrorCode::*;

#[derive(Debug, PartialEq, Copy)]
pub enum LexerErrorCode {
    InvalidSyntax,
    TrailingCharacters,
    UnexpectedEndOfInput,
}

#[derive(Debug, PartialEq, Copy)]
pub struct LexerError {
    code: LexerErrorCode,
    line: usize,
    col: usize,
}

impl LexerError {
    pub fn new(line: usize, col: usize, code: LexerErrorCode) -> LexerError {
        LexerError {
            line: line,
            col: col,
            code: code
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.code {
            InvalidSyntax => {
                write!(f, "Invalid syntax at {}:{}", self.line, self.col)
            },
            TrailingCharacters => {
                write!(f, "Trailing characters at {}:{}", self.line, self.col)
            },
            UnexpectedEndOfInput => {
                write!(f, "Unexpected end of input at {}:{}", self.line, self.col)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LexerError;
    use super::LexerErrorCode::*;

    #[test]
    fn test_descriptions_for_error_codes() {
        let err = LexerError::new(1, 10, InvalidSyntax);
        assert_eq!("Invalid syntax at 1:10", format!("{}", err));
        let err = LexerError::new(2, 13, TrailingCharacters);
        assert_eq!("Trailing characters at 2:13", format!("{}", err));
        let err = LexerError::new(10, 1230, UnexpectedEndOfInput);
        assert_eq!("Unexpected end of input at 10:1230", format!("{}", err));
    }
}
