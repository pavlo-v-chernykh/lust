#[macro_use]
mod macros;
mod token;
mod error;

use std::iter::Peekable;

pub use self::token::{Token, Span};
pub use self::error::{LexerError, LexerErrorCode};

pub type LexerResult = Result<Token, LexerError>;

#[derive(Debug, PartialEq)]
enum LexerState {
    Start,
    ReadList,
    BeforeFinish,
    Finish,
}

pub struct Lexer<I: Iterator> {
    reader: Peekable<I>,
    char: Option<char>,
    line: usize,
    col: usize,
    state: LexerState,
    lvl: isize,
}

impl<I: Iterator<Item=char>> Iterator for Lexer<I> {
    type Item = LexerResult;

    fn next(&mut self) -> Option<LexerResult> {
        match self.state {
            LexerState::BeforeFinish => {
                self.consume_whitespaces();
                match self.char {
                    Some(_) => {
                        Some(self.error(LexerErrorCode::TrailingCharacters))
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
                Some(self.read())
            }
        }
    }
}

impl<I: Iterator<Item=char>> Lexer<I> {
    pub fn new(reader: I) -> Lexer<I> {
        let mut l = Lexer {
            reader: reader.peekable(),
            char: None,
            line: 1,
            col: 0,
            state: LexerState::Start,
            lvl: 0,
        };
        l.bump();
        l
    }

    fn bump(&mut self) {
        self.char = self.reader.next();
        if Some('\n') == self.char {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }

    fn read(&mut self) -> LexerResult {
        loop {
            self.consume_whitespaces();
            match self.state {
                LexerState::Start => {
                    return self.read_at_start()
                },
                LexerState::ReadList => {
                    return self.read_at_read_list()
                },
                _ => {
                    return self.error(LexerErrorCode::InvalidSyntax)
                }
            }
        }
    }

    fn read_at_start(&mut self) -> LexerResult {
        let evt = self.read_next();
        self.state = match evt {
            Ok(Token::ListStart { .. }) => {
                LexerState::ReadList
            },
            Err(_) => {
                LexerState::Finish
            },
            _ => {
                LexerState::BeforeFinish
            },
        };
        evt
    }

    fn read_next(&mut self) -> LexerResult {
        if let Some(c) = self.char {
            match c {
                '-' | '+' => {
                    let mut is_digit = false;
                    if let Some(nc) = self.reader.peek() {
                        is_digit = nc.is_digit(10)
                    }
                    if is_digit {
                        self.read_number()
                    } else {
                        self.read_symbol()
                    }
                },
                'a' ... 'z' | 'A' ... 'Z' | '/' | '*' | '%' | '>' | '<' | '=' => {
                    self.read_symbol()
                },
                '0' ... '9' => {
                    self.read_number()
                },
                '"' => {
                    self.read_string()
                },
                '(' => {
                    let (line, col) = (self.line, self.col);
                    self.bump();
                    Ok(t_list_start!(span!(line, col, self.line, self.col)))
                },
                ')' => {
                    let (line, col) = (self.line, self.col);
                    self.bump();
                    Ok(t_list_end!(span!(line, col, self.line, self.col)))
                },
                _ => {
                    self.error(LexerErrorCode::InvalidSyntax)
                }
            }
        } else {
            self.error(LexerErrorCode::UnexpectedEndOfInput)
        }
    }

    fn read_symbol(&mut self) -> LexerResult {
        let (line, col) = (self.line, self.col);
        let mut res = String::new();

        while let Some(c) = self.char {
            if c.is_whitespace() || c == ')' {
                break
            } else {
                res.push(c)
            }
            self.bump();
        }

        Ok(t_symbol!(res, span!(line, col, self.line, self.col)))
    }

    fn read_number(&mut self) -> LexerResult {
        let (line, col) = (self.line, self.col);

        let mut neg = false;
        if Some('-') == self.char {
            neg = true;
            self.bump();
        } else if Some('+') == self.char {
            self.bump();
        }

        let mut accum = 0_f64;

        while let Some(c) = self.char {
            match c {
                '0' ... '9' => {
                    accum *= 10_f64;
                    accum += c.to_digit(10).unwrap() as f64;
                    self.bump()
                },
                '.' => {
                    self.bump();

                    let mut dec = 1.0;
                    while let Some(c) = self.char {
                        match c {
                            '0' ... '9' => {
                                dec /= 10.0;
                                accum += dec * c.to_digit(10).unwrap() as f64;
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
                    return self.error(LexerErrorCode::InvalidSyntax)
                }
            }
        }

        if neg {
            accum *= -1_f64;
        }

        Ok(t_number!(accum, span!(line, col, self.line, self.col)))
    }

    fn read_string(&mut self) -> LexerResult {
        let (line, col) = (self.line, self.col);

        let mut res = String::new();

        self.bump();

        while let Some(c) = self.char {
            if c == '"' {
                break
            } else {
                res.push(c)
            }
            self.bump();
        }

        self.bump();

        Ok(t_string!(res, span!(line, col, self.line, self.col)))
    }

    fn read_at_read_list(&mut self) -> LexerResult {
        let evt = self.read_next();
        match evt {
            Ok(Token::ListStart { .. }) => {
                self.lvl += 1;
            },
            Ok(Token::ListEnd { .. }) => {
                if self.lvl > 0 {
                    self.lvl -= 1;
                } else {
                    self.state = LexerState::BeforeFinish
                }
            },
            Err(_) => {
                self.state = LexerState::Finish
            },
            _ => {}
        };
        evt
    }

    fn error(&mut self, code: LexerErrorCode) -> LexerResult {
        self.state = LexerState::Finish;
        Err(LexerError::new(self.line, self.col, code))
    }

    fn consume_whitespaces(&mut self) {
        while let Some(c) = self.char {
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
    use super::{Lexer, LexerResult, LexerError, LexerErrorCode};

    #[test]
    fn test_read_nil() {
        let sym_name = "nil";
        let t_sym = t_symbol!(sym_name, span!(1, 1, 1, 4));
        let mut lexer = Lexer::new(sym_name.chars());
        assert_eq!(Some(Ok(t_sym)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_integer_as_float() {
        let mut lexer = Lexer::new("64".chars());
        let t_num = t_number!(64_f64, span!(1, 1, 1, 3));
        assert_eq!(Some(Ok(t_num)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_float() {
        let mut lexer = Lexer::new("64.5".chars());
        let t_num = t_number!(64.5, span!(1, 1, 1, 5));
        assert_eq!(Some(Ok(t_num)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_string() {
        let s = "rust is beautiful";
        let actual_input = format!(r#""{}""#, s);
        let mut lexer = Lexer::new(actual_input.chars());
        let t_str = t_string!(s, span!(1, 1, 1, 20));
        assert_eq!(Some(Ok(t_str)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_symbol() {
        let sym_name = "my-symbol";
        let mut lexer = Lexer::new(sym_name.chars());
        let t_sym = t_symbol!(sym_name, span!(1, 1, 1, 10));
        assert_eq!(Some(Ok(t_sym)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_incorrect_symbol_starting_with_digit() {
        let sym_name = "6-my-incorrect-symbol";
        let mut lexer = Lexer::new(sym_name.chars());
        let expected_result = Some(Err(LexerError::new(1, 2, LexerErrorCode::InvalidSyntax)));
        assert_eq!(expected_result, lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_explicitly_positive_number() {
        let mut lexer = Lexer::new("+1".chars());
        let t_num = t_number!(1_f64, span!(1, 1, 1, 3));
        let expected_result = Some(Ok(t_num));
        assert_eq!(expected_result, lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_explicitly_negative_number() {
        let mut lexer = Lexer::new("-1".chars());
        let t_num = t_number!(-1_f64, span!(1, 1, 1, 3));
        let expected_result = Some(Ok(t_num));
        assert_eq!(expected_result, lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_dense_expression() {
        let lexer = Lexer::new("(def a 1)".chars());
        let expected_result = vec![Ok(t_list_start!(span!(1, 1, 1, 2))),
                                   Ok(t_symbol!("def", span!(1, 2, 1, 5))),
                                   Ok(t_symbol!("a", span!(1, 6, 1, 7))),
                                   Ok(t_number!(1_f64, span!(1, 8, 1, 9))),
                                   Ok(t_list_end!(span!(1, 9, 1, 10)))];
        assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
    }

    #[test]
    fn test_read_sparse_expression() {
        let lexer = Lexer::new(" ( \n def a\n1)   \n".chars());
        let expected_result = vec![Ok(t_list_start!(span!(1, 2, 1, 3))),
                                   Ok(t_symbol!("def", span!(2, 3, 2, 6))),
                                   Ok(t_symbol!("a", span!(2, 7, 3, 1))),
                                   Ok(t_number!(1_f64, span!(3, 2, 3, 3))),
                                   Ok(t_list_end!(span!(3, 3, 3, 4)))];
        assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
    }

    #[test]
    fn test_read_nested_list_expressions() {
        let lexer = Lexer::new("(def a (+ 1 1))".chars());
        let expected_result = vec![Ok(t_list_start!(span!(1, 1, 1, 2))),
                                   Ok(t_symbol!("def", span!(1, 2, 1, 5))),
                                   Ok(t_symbol!("a", span!(1, 6, 1, 7))),
                                   Ok(t_list_start!(span!(1, 8, 1, 9))),
                                   Ok(t_symbol!("+", span!(1, 9, 1, 10))),
                                   Ok(t_number!(1_f64, span!(1, 11, 1, 12))),
                                   Ok(t_number!(1_f64, span!(1, 13, 1, 14))),
                                   Ok(t_list_end!(span!(1, 14, 1, 15))),
                                   Ok(t_list_end!(span!(1, 15, 1, 16)))];
        assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
    }

    #[test]
    fn test_read_list_of_symbols() {
        let lexer = Lexer::new("(+ - / * % > < = a b c z A X Y Z)".chars());
        let expected_result = vec![Ok(t_list_start!(span!(1, 1, 1, 2))),
                                   Ok(t_symbol!("+", span!(1, 2, 1, 3))),
                                   Ok(t_symbol!("-", span!(1, 4, 1, 5))),
                                   Ok(t_symbol!("/", span!(1, 6, 1, 7))),
                                   Ok(t_symbol!("*", span!(1, 8, 1, 9))),
                                   Ok(t_symbol!("%", span!(1, 10, 1, 11))),
                                   Ok(t_symbol!(">", span!(1, 12, 1, 13))),
                                   Ok(t_symbol!("<", span!(1, 14, 1, 15))),
                                   Ok(t_symbol!("=", span!(1, 16, 1, 17))),
                                   Ok(t_symbol!("a", span!(1, 18, 1, 19))),
                                   Ok(t_symbol!("b", span!(1, 20, 1, 21))),
                                   Ok(t_symbol!("c", span!(1, 22, 1, 23))),
                                   Ok(t_symbol!("z", span!(1, 24, 1, 25))),
                                   Ok(t_symbol!("A", span!(1, 26, 1, 27))),
                                   Ok(t_symbol!("X", span!(1, 28, 1, 29))),
                                   Ok(t_symbol!("Y", span!(1, 30, 1, 31))),
                                   Ok(t_symbol!("Z", span!(1, 32, 1, 33))),
                                   Ok(t_list_end!(span!(1, 33, 1, 34)))];
        assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
    }
}
