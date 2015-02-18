use std::str::FromStr;
use common::Atom;

enum LexerErrorCode {
    InvalidSyntax,
    TrailingCharacters,
    EOFWhileReadingToken,
}

enum LexerError {
    SyntaxError {
        code: LexerErrorCode,
        line: usize,
        col: usize,
    }
}

#[derive(Debug, PartialEq)]
enum ParseAtomError {
    IncorrectSymbolName
}

impl FromStr for Atom {
    type Err = ParseAtomError;

    fn from_str(s: &str) -> Result<Atom, ParseAtomError> {
        match s.parse::<f64>() {
            Ok(f) => {
                Ok(Atom::Number(f))
            },
            _ => {
                match s.chars().next() {
                    Some(c) if !c.is_numeric() => {
                        Ok(Atom::Symbol(s.to_string()))
                    },
                    _ => {
                        Err(ParseAtomError::IncorrectSymbolName)
                    }
                }
            }
        }
    }
}

enum Token {
    ListStart,
    ListEnd,
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
    Error(LexerError),
}

struct Stack {
    field: Option<usize>
}

impl Stack {
    fn new() -> Stack {
        Stack {
            field: None
        }
    }
}

enum LexerState {
    Start,
    ListStart,
    ListEnd,
    BeforeFinish,
    Finish,
}

pub struct Lexer<T> {
    reader: T,
    cur_char: Option<char>,
    line: usize,
    col: usize,
    stack: Stack,
    state: LexerState
}

impl<T: Iterator<Item=char>> Iterator for Lexer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.state {
            LexerState::Finish => {
                None
            },
            LexerState::BeforeFinish => {
                self.read_whitespaces();
                self.cur_char
                    .and(Some(self.error_token(LexerErrorCode::TrailingCharacters)))
                    .or_else(|| { self.state = LexerState::Finish; None })
            },
            _ => {
                Some(self.read())
            }
        }
    }
}

impl<T: Iterator<Item=char>> Lexer<T> {
    fn new(reader: T) -> Lexer<T> {
        let mut l = Lexer {
            reader: reader,
            cur_char: None,
            line: 1,
            col: 0,
            stack: Stack::new(),
            state: LexerState::Start
        };
        l.bump();
        l
    }

    fn read(&mut self) -> Token {
        loop {
            self.read_whitespaces();
            match self.state {
                LexerState::Start => {
                    return self.read_start()
                },
                _ => {
                    return self.error_token(LexerErrorCode::InvalidSyntax)
                }
            }
        }
    }

    fn read_start(&mut self) -> Token {
        let token = self.read_token();
        self.state = match token {
            Token::Error(_) => {
                LexerState::Finish
            },
            Token::ListStart => {
                LexerState::ListStart
            },
            _ => LexerState::BeforeFinish,
        };
        token
    }

    fn read_token(&mut self) -> Token {
        if let Some(c) = self.cur_char {
            match c {
                'n' => {
                    self.read_ident("il", Token::Nil)
                },
                't' => {
                    self.read_ident("rue", Token::Boolean(true))
                },
                'f' => {
                    self.read_ident("alse", Token::Boolean(false))
                },
                '0' ... '9' | '-' => {
                    self.read_number()
                },
                '"' => {
                    self.read_str().unwrap_or_else(|e| Token::Error(e))
                },
                '(' => {
                    self.bump();
                    Token::ListStart
                }
                _ => {
                    self.error_token(LexerErrorCode::InvalidSyntax)
                }
            }
        } else {
            self.error_token(LexerErrorCode::EOFWhileReadingToken)
        }
    }

    fn read_ident(&mut self, ident: &str, token: Token) -> Token {
        if ident.chars().all(|c| { self.bump(); Some(c) == self.cur_char }) {
            self.bump();
            token
        } else {
            Token::Error(LexerError::SyntaxError {
                code: LexerErrorCode::InvalidSyntax,
                line: self.line,
                col: self.col,
            })
        }
    }

    fn read_number(&mut self) -> Token {
        Token::Number(0.)
    }

    fn read_str(&mut self) -> Result<Token, LexerError> {
        Ok(Token::String("".to_string()))
    }

    fn read_whitespaces(&mut self) {
        while let Some(ch) = self.cur_char {
            if [' ', '\n', '\r', '\t'].contains(&ch) {
                self.bump()
            }
        }
    }

    fn bump(&mut self) {
        self.cur_char = self.reader.next();
        if Some('\n') == self.cur_char {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }

    fn error_token(&mut self, ec: LexerErrorCode) -> Token {
        self.state = LexerState::Finish;
        Token::Error(LexerError::SyntaxError {
            code: ec,
            line: self.line,
            col: self.col
        })
    }
}

fn tokenize(s: &str) -> Vec<String> {
    s.replace("("," ( ")
        .replace(")", " ) ")
        .replace("\n", " ")
        .split(' ')
        .filter(|s| { !s.is_empty() })
        .map(|s| { s.to_string() })
        .collect()
}


#[cfg(test)]
mod tests {
    use common::Atom::{self, Symbol, Number};
    use super::Lexer;
    use super::ParseAtomError::IncorrectSymbolName;
    use super::tokenize;

    #[test]
    fn test_read_integer() {
        assert_eq!(Number(64f64), "64".parse::<Atom>().ok().unwrap())
    }

    #[test]
    fn test_read_float() {
        assert_eq!(Number(64.5), "64.5".parse::<Atom>().ok().unwrap())
    }

    #[test]
    fn test_read_symbol() {
        assert_eq!(Symbol("name".to_string()), "name".parse::<Atom>().ok().unwrap())
    }

    #[test]
    fn test_read_incorrect_symbol_starting_with_digit() {
        assert_eq!(IncorrectSymbolName, "6name".parse::<Atom>().err().unwrap())
    }

    #[test]
    fn test_tokenize_dense_expression() {
        let expected_result = ["(", "def", "a", "1", ")"]
                                .iter()
                                .map(|s| { s.to_string() })
                                .collect();
        assert_eq!(expected_result, tokenize("(def a 1)"))
    }

    #[test]
    fn test_tokenize_sread_expression() {
        let expected_result = ["(", "def", "a", "1", ")"]
                                .iter()
                                .map(|s| { s.to_string() })
                                .collect();
        assert_eq!(expected_result, tokenize(" ( \n def a\n1)   \n"))
    }
}
