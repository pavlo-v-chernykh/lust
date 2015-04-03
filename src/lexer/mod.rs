mod token;
mod error;
#[cfg(test)]
mod tests;

use std::iter::Peekable;

pub use self::token::{Token, Span};
pub use self::error::LexerError;

pub type LexerResult = Result<Token, LexerError>;

pub struct Lexer<I: Iterator> {
    reader: Peekable<I>,
    char: Option<char>,
    line: usize,
    col: usize,
    is_finished: bool,
}

impl<I: Iterator<Item=char>> Iterator for Lexer<I> {
    type Item = LexerResult;

    fn next(&mut self) -> Option<LexerResult> {
        self.read()
    }
}

impl<I: Iterator<Item=char>> Lexer<I> {
    pub fn new(reader: I) -> Lexer<I> {
        let mut l = Lexer {
            reader: reader.peekable(),
            char: None,
            line: 1,
            col: 0,
            is_finished: false,
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

    fn read(&mut self) -> Option<LexerResult> {
        self.consume_whitespaces();
        if self.is_finished {
            None
        } else {
            self.read_next()
        }
    }

    fn read_next(&mut self) -> Option<LexerResult> {
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
                'a' ... 'z' | 'A' ... 'Z' |
                '/' | '*' | '%' | '>' | '<' | '=' |
                '&' => {
                    self.read_symbol()
                },
                ':' => {
                    self.read_keyword()
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
                    Some(Ok(t_list_start!(span!(line, col, line, col + 1))))
                },
                ')' => {
                    let (line, col) = (self.line, self.col);
                    self.bump();
                    Some(Ok(t_list_end!(span!(line, col, line, col + 1))))
                },
                '[' => {
                    let (line, col) = (self.line, self.col);
                    self.bump();
                    Some(Ok(t_vec_start!(span!(line, col, line, col + 1))))
                },
                ']' => {
                    let (line, col) = (self.line, self.col);
                    self.bump();
                    Some(Ok(t_vec_end!(span!(line, col, line, col + 1))))
                },
                '\'' => {
                    let (line, col) = (self.line, self.col);
                    self.bump();
                    Some(Ok(t_quote![span![line, col, line, col + 1]]))
                },
                '~' => {
                    let (line, col) = (self.line, self.col);
                    self.bump();
                    if Some('@') == self.char {
                        self.bump();
                        Some(Ok(t_unquote_splicing![span![line, col, line, col + 2]]))
                    } else {
                        Some(Ok(t_unquote![span![line, col, line, col + 1]]))
                    }
                },
                _ => {
                    Some(self.error())
                }
            }
        } else {
            None
        }
    }

    fn read_symbol(&mut self) -> Option<LexerResult> {
        let (line, col) = (self.line, self.col);
        let mut symbol = String::new();

        while let Some(c) = self.char {
            if c == ')' ||
               c == ']' ||
               c.is_whitespace() {
                break
            } else {
                symbol.push(c)
            }
            self.bump();
        }

        Some(Ok(t_symbol!(symbol, span!(line, col, self.line, self.col))))
    }

    fn read_keyword(&mut self) -> Option<LexerResult> {
        let (line, col) = (self.line, self.col);
        let mut keyword = String::new();

        while let Some(c) = self.char {
            if c == ')' ||
               c == ']' ||
               c.is_whitespace() {
                break
            } else {
                keyword.push(c)
            }
            self.bump();
        }

        Some(Ok(t_keyword!(keyword, span!(line, col, self.line, self.col))))
    }

    fn read_number(&mut self) -> Option<LexerResult> {
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
                '(' | ')' | '[' | ']' => {
                    break
                },
                c if c.is_whitespace() => {
                    break
                },
                _ => {
                    return Some(self.error())
                }
            }
        }

        if neg {
            accum *= -1_f64;
        }

        Some(Ok(t_number!(accum, span!(line, col, self.line, self.col))))
    }

    fn read_string(&mut self) -> Option<LexerResult> {
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

        Some(Ok(t_string!(res, span!(line, col, self.line, self.col))))
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

    fn error(&mut self) -> LexerResult {
        self.is_finished = true;
        Err(LexerError::new(self.line, self.col))
    }
}
