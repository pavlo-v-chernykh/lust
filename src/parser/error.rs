use std::{error, fmt};
use lexer::{Token, LexerError};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
    LexerError(LexerError),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ParserError::UnexpectedToken(ref t) => {
                write!(f, "{} {}", error::Error::description(self), t)
            },
            &ParserError::LexerError(ref e) => {
                write!(f, "{}", e)
            },
            _ => {
                write!(f, "{}", error::Error::description(self))
            },
        }
    }
}

impl error::Error for ParserError {
    fn description(&self) -> &str {
        match self {
            &ParserError::UnexpectedToken(_) => {
                "Unexpected token"
            },
            &ParserError::UnexpectedEndOfInput => {
                "Unexpected end of input"
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

#[cfg(test)]
mod tests {
    use lexer::LexerError;
    use lexer::LexerErrorCode::InvalidSyntax;
    use super::ParserError;

    #[test]
    fn test_descriptions_for_error_codes() {
        let err = ParserError::UnexpectedToken(t_list_end!(span!(1, 1, 1, 2)));
        assert_eq!("Unexpected token 'List End' at 1:1-1:2", format!("{}", err));
        let err = ParserError::UnexpectedEndOfInput;
        assert_eq!("Unexpected end of input", format!("{}", err));
        let err = ParserError::LexerError(LexerError::new(1, 10, InvalidSyntax));
        assert_eq!("Invalid syntax at 1:10", format!("{}", err));
    }
}
