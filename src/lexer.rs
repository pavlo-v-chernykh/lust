use std::str::FromStr;
use common::Atom;

#[derive(Debug, PartialEq)]
enum LexerErrorCode {
    InvalidSyntax,
    TrailingCharacters,
    EOFWhileReadingToken,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    String(String),
    Symbol(String),
    Error(LexerError),
}

enum LexerState {
    Start,
    BeforeFinish,
    Finish,
}

pub struct Lexer<T> {
    reader: T,
    cur_char: Option<char>,
    line: usize,
    col: usize,
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
            _ => LexerState::BeforeFinish,
        };
        token
    }

    fn read_token(&mut self) -> Token {
        if let Some(c) = self.cur_char {
            match c {
                'a' ... 'z' | 'A' ... 'Z' => {
                    self.read_symbol()
                },
                '0' ... '9' | '-' => {
                    self.read_number()
                },
                '"' => {
                    self.read_string()
                },
                _ => {
                    self.error_token(LexerErrorCode::InvalidSyntax)
                }
            }
        } else {
            self.error_token(LexerErrorCode::EOFWhileReadingToken)
        }
    }

    fn read_symbol(&mut self) -> Token {
        let mut res = String::new();

        while let Some(c) = self.cur_char {
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    break
                },
                _ => {
                    res.push(c);
                }
            }
            self.bump();
        }

        Token::Symbol(res)
    }

    fn read_number(&mut self) -> Token {
        let mut neg = false;

        if let Some('-') = self.cur_char {
            neg = true;
            self.bump();
        }

        let mut accum = 0_f64;

        while let Some(c) = self.cur_char {
            match c {
                '0' ... '9' => {
                    let c_as_usize = (c as usize) - ('0' as usize);
                    accum *= 10_f64;
                    accum += c_as_usize as f64;
                    self.bump()
                },
                '.' => {
                    self.bump();

                    let mut dec = 1.0;
                    while let Some(c) = self.cur_char {
                        match c {
                            '0' ... '9' => {
                                let c_as_usize = (c as usize) - ('0' as usize);
                                dec /= 10.0;
                                accum += (c_as_usize as f64) * dec;
                                self.bump();
                            },
                            _ => {
                                break
                            }
                        }
                    }
                },
                ' ' | '\t' | '\r' | '\n' => {
                    break
                },
                _ => {
                    return self.error_token(LexerErrorCode::InvalidSyntax)
                }
            }
        }

        if neg {
            accum *= -1_f64;
        }

        Token::Number(accum)
    }

    fn read_string(&mut self) -> Token {
        let mut res = String::new();

        self.bump();

        while let Some(c) = self.cur_char {
            match c {
                '"' => {
                    break
                },
                _ => {
                    res.push(c);
                }
            }
            self.bump();
        }

        self.bump();

        Token::String(res)
    }

    fn read_whitespaces(&mut self) {
        while let Some(c) = self.cur_char {
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    self.bump();
                },
                _ => {
                    break;
                }
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
    use super::{Lexer, Token, LexerError, LexerErrorCode};
    use super::tokenize;

    #[test]
    fn test_read_nil() {
        let sym_name = "nil".to_string();
        let mut lexer = Lexer::new(sym_name.chars());
        assert_eq!(Some(Token::Symbol(sym_name.clone())), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_integer_as_float() {
        let mut lexer = Lexer::new("64".chars());
        assert_eq!(Some(Token::Number(64_f64)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_float() {
        let mut lexer = Lexer::new("64.5".chars());
        assert_eq!(Some(Token::Number(64.5)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_string() {
        let s = "rust is beautiful".to_string();
        let actual_input = format!(r#""{}""#, s);
        let mut lexer = Lexer::new(actual_input.chars());
        assert_eq!(Some(Token::String(s)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_symbol() {
        let sym_name = "my-symbol".to_string();
        let mut lexer = Lexer::new(sym_name.chars());
        assert_eq!(Some(Token::Symbol(sym_name.clone())), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_incorrect_symbol_starting_with_digit() {
        let sym_name = "6-my-incorrect-symbol".to_string();
        let mut lexer = Lexer::new(sym_name.chars());
        let expected_result = Some(Token::Error(LexerError::SyntaxError {
            code: LexerErrorCode::InvalidSyntax,
            line: 1,
            col: 2
        }));
        assert_eq!(expected_result, lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_dense_expression() {
        let expected_result = ["(", "def", "a", "1", ")"]
                                .iter()
                                .map(|s| { s.to_string() })
                                .collect();
        assert_eq!(expected_result, tokenize("(def a 1)"))
    }

    #[test]
    fn test_read_sparse_expression() {
        let expected_result = ["(", "def", "a", "1", ")"]
                                .iter()
                                .map(|s| { s.to_string() })
                                .collect();
        assert_eq!(expected_result, tokenize(" ( \n def a\n1)   \n"))
    }
}
