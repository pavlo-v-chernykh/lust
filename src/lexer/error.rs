use std::{error, fmt};

use self::LexerErrorCode::*;

#[derive(Debug, PartialEq, Copy)]
pub enum LexerErrorCode {
    InvalidSyntax,
    TrailingCharacters,
    EOFWhileReadingToken,
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
        write!(f, "{} detected at {}:{}", error::Error::description(self), self.line, self.col)
    }
}

impl error::Error for LexerError {
    fn description(&self) -> &str {
        match self.code {
            InvalidSyntax => {
                "Invalid syntax"
            },
            TrailingCharacters => {
                "Trailing characters"
            },
            EOFWhileReadingToken => {
                "Unexpected end of file"
            }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::LexerError;
    use super::LexerErrorCode::*;

    #[test]
    fn test_descriptions_for_error_codes() {
        let err = LexerError::new(1, 10, InvalidSyntax);
        assert_eq!("Invalid syntax detected at 1:10", format!("{}", err));
        let err = LexerError::new(2, 13, TrailingCharacters);
        assert_eq!("Trailing characters detected at 2:13", format!("{}", err));
        let err = LexerError::new(10, 1230, EOFWhileReadingToken);
        assert_eq!("Unexpected end of file detected at 10:1230", format!("{}", err));
    }
}
