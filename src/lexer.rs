use std::iter::Peekable;
use token::Token;

#[derive(Debug, PartialEq, Copy)]
enum LexerErrorCode {
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
            Ok(Token::ListStart) => {
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
                    self.bump();
                    Ok(Token::ListStart)
                },
                ')' => {
                    self.bump();
                    Ok(Token::ListEnd)
                },
                _ => {
                    self.error(LexerErrorCode::InvalidSyntax)
                }
            }
        } else {
            self.error(LexerErrorCode::EOFWhileReadingToken)
        }
    }

    fn read_symbol(&mut self) -> LexerResult {
        let mut res = String::new();

        while let Some(c) = self.char {
            if c.is_whitespace() || c == ')' {
                break
            } else {
                res.push(c)
            }
            self.bump();
        }

        Ok(Token::Symbol(res))
    }

    fn read_number(&mut self) -> LexerResult {
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

        Ok(Token::Number(accum))
    }

    fn read_string(&mut self) -> LexerResult {
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

        Ok(Token::String(res))
    }

    fn read_at_read_list(&mut self) -> LexerResult {
        let evt = self.read_next();
        match evt {
            Ok(Token::ListStart) => {
                self.lvl += 1;
            },
            Ok(Token::ListEnd) => {
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

    fn error(&mut self, ec: LexerErrorCode) -> LexerResult {
        self.state = LexerState::Finish;
        Err(LexerError {
            code: ec,
            line: self.line,
            col: self.col
        })
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
        let mut lexer = Lexer::new(sym_name.chars());
        assert_eq!(Some(Ok(t_symbol!(sym_name))), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_integer_as_float() {
        let mut lexer = Lexer::new("64".chars());
        assert_eq!(Some(Ok(t_number!(64_f64))), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_float() {
        let mut lexer = Lexer::new("64.5".chars());
        assert_eq!(Some(Ok(t_number!(64.5))), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_string() {
        let s = "rust is beautiful";
        let actual_input = format!(r#""{}""#, s);
        let mut lexer = Lexer::new(actual_input.chars());
        assert_eq!(Some(Ok(t_string!(s))), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_symbol() {
        let sym_name = "my-symbol";
        let mut lexer = Lexer::new(sym_name.chars());
        assert_eq!(Some(Ok(t_symbol!(sym_name))), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_incorrect_symbol_starting_with_digit() {
        let sym_name = "6-my-incorrect-symbol";
        let mut lexer = Lexer::new(sym_name.chars());
        let expected_result = Some(Err(LexerError {
            code: LexerErrorCode::InvalidSyntax,
            line: 1,
            col: 2
        }));
        assert_eq!(expected_result, lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_explicitly_positive_number() {
        let mut lexer = Lexer::new("+1".chars());
        let expected_result = Some(Ok(t_number!(1_f64)));
        assert_eq!(expected_result, lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_explicitly_negative_number() {
        let mut lexer = Lexer::new("-1".chars());
        let expected_result = Some(Ok(t_number!(-1_f64)));
        assert_eq!(expected_result, lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_read_dense_expression() {
        let lexer = Lexer::new("(def a 1)".chars());
        let expected_result = vec![Ok(t_list_start![]),
                                   Ok(t_symbol!("def")),
                                   Ok(t_symbol!("a")),
                                   Ok(t_number!(1_f64)),
                                   Ok(t_list_end![])];
        assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
    }

    #[test]
    fn test_read_sparse_expression() {
        let lexer = Lexer::new(" ( \n def a\n1)   \n".chars());
        let expected_result = vec![Ok(t_list_start![]),
                                   Ok(t_symbol!("def")),
                                   Ok(t_symbol!("a")),
                                   Ok(t_number!(1_f64)),
                                   Ok(t_list_end![])];
        assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
    }

    #[test]
    fn test_read_nested_list_expressions() {
        let lexer = Lexer::new("(def a (+ 1 1))".chars());
        let expected_result = vec![Ok(t_list_start![]),
                                   Ok(t_symbol!("def")),
                                   Ok(t_symbol!("a")),
                                   Ok(t_list_start![]),
                                   Ok(t_symbol!("+")),
                                   Ok(t_number!(1_f64)),
                                   Ok(t_number!(1_f64)),
                                   Ok(t_list_end![]),
                                   Ok(t_list_end![])];
        assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
    }

    #[test]
    fn test_read_list_of_symbols() {
        let lexer = Lexer::new("(+ - / * % > < = a b c z A X Y Z)".chars());
        let expected_result = vec![Ok(t_list_start![]),
                                   Ok(t_symbol!("+")),
                                   Ok(t_symbol!("-")),
                                   Ok(t_symbol!("/")),
                                   Ok(t_symbol!("*")),
                                   Ok(t_symbol!("%")),
                                   Ok(t_symbol!(">")),
                                   Ok(t_symbol!("<")),
                                   Ok(t_symbol!("=")),
                                   Ok(t_symbol!("a")),
                                   Ok(t_symbol!("b")),
                                   Ok(t_symbol!("c")),
                                   Ok(t_symbol!("z")),
                                   Ok(t_symbol!("A")),
                                   Ok(t_symbol!("X")),
                                   Ok(t_symbol!("Y")),
                                   Ok(t_symbol!("Z")),
                                   Ok(t_list_end![])];
        assert_eq!(expected_result, lexer.collect::<Vec<LexerResult>>());
    }
}
