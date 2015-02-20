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
enum Token {
    Number(f64),
    String(String),
    Symbol(String),
    ListStart,
    ListEnd,
}

#[derive(Debug, PartialEq)]
enum LexerEvent {
    Token(Token),
    Error(LexerError),
}

#[derive(Debug, PartialEq)]
enum LexerState {
    Start,
    ReadList,
    BeforeFinish,
    Finish,
}

pub struct Lexer<T> {
    reader: T,
    cur_char: Option<char>,
    line: usize,
    col: usize,
    state: LexerState,
    lvl: isize,
}

impl<T: Iterator<Item=char>> Iterator for Lexer<T> {
    type Item = LexerEvent;

    fn next(&mut self) -> Option<LexerEvent> {
        match self.state {
            LexerState::BeforeFinish => {
                self.consume_whitespaces();
                match self.cur_char {
                    Some(_) => {
                        Some(self.emit_syntax_error(LexerErrorCode::TrailingCharacters))
                    },
                    None => {
                        self.state = LexerState::Finish;
                        None
                    }
                }
            },
            LexerState::Finish => {
                None
            },
            _ => {
                Some(self.emit())
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
            state: LexerState::Start,
            lvl: 0,
        };
        l.bump();
        l
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

    fn emit(&mut self) -> LexerEvent {
        loop {
            self.consume_whitespaces();
            match self.state {
                LexerState::Start => {
                    return self.emit_at_start()
                },
                LexerState::ReadList => {
                    return self.emit_at_read_list()
                },
                _ => {
                    return self.emit_syntax_error(LexerErrorCode::InvalidSyntax)
                }
            }
        }
    }

    fn emit_at_start(&mut self) -> LexerEvent {
        let evt = self.emit_next();
        self.state = match evt {
            LexerEvent::Token(Token::ListStart) => {
                LexerState::ReadList
            },
            LexerEvent::Error(_) => {
                LexerState::Finish
            },
            _ => {
                LexerState::BeforeFinish
            },
        };
        evt
    }

    fn emit_next(&mut self) -> LexerEvent {
        if let Some(c) = self.cur_char {
            match c {
                'a' ... 'z' | 'A' ... 'Z' | '+' => {
                    self.emit_symbol()
                },
                '0' ... '9' | '-' => {
                    self.emit_number()
                },
                '"' => {
                    self.emit_string()
                },
                '(' => {
                    self.bump();
                    self.emit_token(Token::ListStart)
                },
                ')' => {
                    self.bump();
                    self.emit_token(Token::ListEnd)
                },
                _ => {
                    self.emit_syntax_error(LexerErrorCode::InvalidSyntax)
                }
            }
        } else {
            self.emit_syntax_error(LexerErrorCode::EOFWhileReadingToken)
        }
    }

    fn emit_symbol(&mut self) -> LexerEvent {
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

        self.emit_token(Token::Symbol(res))
    }

    fn emit_number(&mut self) -> LexerEvent {
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
                ' ' | '\t' | '\r' | '\n' | ')' => {
                    break
                },
                _ => {
                    return self.emit_syntax_error(LexerErrorCode::InvalidSyntax)
                }
            }
        }

        if neg {
            accum *= -1_f64;
        }

        self.emit_token(Token::Number(accum))
    }

    fn emit_string(&mut self) -> LexerEvent {
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

        self.emit_token(Token::String(res))
    }

    fn emit_at_read_list(&mut self) -> LexerEvent {
        let evt = self.emit_next();
        match evt {
            LexerEvent::Token(Token::ListStart) => {
                self.lvl += 1;
            },
            LexerEvent::Token(Token::ListEnd) => {
                if self.lvl > 0 {
                    self.lvl -= 1;
                } else {
                    self.state = LexerState::BeforeFinish
                }
            },
            LexerEvent::Error(_) => {
                self.state = LexerState::Finish
            },
            _ => {}
        };
        evt
    }

    fn emit_token(&mut self, token: Token) -> LexerEvent {
        LexerEvent::Token(token)
    }

    fn emit_syntax_error(&mut self, ec: LexerErrorCode) -> LexerEvent {
        self.state = LexerState::Finish;
        LexerEvent::Error(LexerError::SyntaxError {
            code: ec,
            line: self.line,
            col: self.col
        })
    }

    fn consume_whitespaces(&mut self) {
        while let Some(c) = self.cur_char {
            if c.is_whitespace() {
                self.bump()
            } else {
                break
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, LexerEvent, Token, LexerError, LexerErrorCode};

    #[test]
    fn test_read_nil() {
        let sym_name = "nil".to_string();
        let mut lexer = Lexer::new(sym_name.chars());
        assert_eq!(Some(LexerEvent::Token(Token::Symbol(sym_name.clone()))), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_integer_as_float() {
        let mut lexer = Lexer::new("64".chars());
        assert_eq!(Some(LexerEvent::Token(Token::Number(64_f64))), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_float() {
        let mut lexer = Lexer::new("64.5".chars());
        assert_eq!(Some(LexerEvent::Token(Token::Number(64.5))), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_string() {
        let s = "rust is beautiful".to_string();
        let actual_input = format!(r#""{}""#, s);
        let mut lexer = Lexer::new(actual_input.chars());
        assert_eq!(Some(LexerEvent::Token(Token::String(s))), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_symbol() {
        let sym_name = "my-symbol".to_string();
        let mut lexer = Lexer::new(sym_name.chars());
        assert_eq!(Some(LexerEvent::Token(Token::Symbol(sym_name.clone()))), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_incorrect_symbol_starting_with_digit() {
        let sym_name = "6-my-incorrect-symbol".to_string();
        let mut lexer = Lexer::new(sym_name.chars());
        let expected_result = Some(LexerEvent::Error(LexerError::SyntaxError {
            code: LexerErrorCode::InvalidSyntax,
            line: 1,
            col: 2
        }));
        assert_eq!(expected_result, lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_dense_expression() {
        let lexer = Lexer::new("(def a 1)".chars());
        let expected_result = vec![LexerEvent::Token(Token::ListStart),
                                   LexerEvent::Token(Token::Symbol("def".to_string())),
                                   LexerEvent::Token(Token::Symbol("a".to_string())),
                                   LexerEvent::Token(Token::Number(1_f64)),
                                   LexerEvent::Token(Token::ListEnd)];
        assert_eq!(expected_result, lexer.collect::<Vec<LexerEvent>>());
    }

    #[test]
    fn test_read_sparse_expression() {
        let lexer = Lexer::new(" ( \n def a\n1)   \n".chars());
        let expected_result = vec![LexerEvent::Token(Token::ListStart),
                                   LexerEvent::Token(Token::Symbol("def".to_string())),
                                   LexerEvent::Token(Token::Symbol("a".to_string())),
                                   LexerEvent::Token(Token::Number(1_f64)),
                                   LexerEvent::Token(Token::ListEnd)];
        assert_eq!(expected_result, lexer.collect::<Vec<LexerEvent>>());
    }

    #[test]
    fn test_read_nested_list_expressions() {
        let lexer = Lexer::new("(def a (+ 1 1))".chars());
        let expected_result = vec![LexerEvent::Token(Token::ListStart),
                                   LexerEvent::Token(Token::Symbol("def".to_string())),
                                   LexerEvent::Token(Token::Symbol("a".to_string())),
                                   LexerEvent::Token(Token::ListStart),
                                   LexerEvent::Token(Token::Symbol("+".to_string())),
                                   LexerEvent::Token(Token::Number(1_f64)),
                                   LexerEvent::Token(Token::Number(1_f64)),
                                   LexerEvent::Token(Token::ListEnd),
                                   LexerEvent::Token(Token::ListEnd)];
        assert_eq!(expected_result, lexer.collect::<Vec<LexerEvent>>());
    }
}
