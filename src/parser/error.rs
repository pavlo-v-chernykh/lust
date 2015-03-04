use std::{error, fmt};
use lexer::LexerError;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken,
    EOFWhileParsingExpression,
    LexerError(LexerError),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} detected", error::Error::description(self))
    }
}

impl error::Error for ParserError {
    fn description(&self) -> &str {
        match self {
            &ParserError::UnexpectedToken => {
                "Unexpected token"
            },
            &ParserError::EOFWhileParsingExpression => {
                "Unexpected end of file"
            },
            &ParserError::LexerError(ref e) => {
                e.description()
            },
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &ParserError::LexerError(ref e) => {
                Some(e)
            },
            _ => {
                None
            }
        }
    }
}
