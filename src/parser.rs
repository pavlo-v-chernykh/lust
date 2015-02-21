use common::{Atom, Sexp};
use lexer::{Lexer, LexerEvent, Token, LexerError};

#[derive(Debug, PartialEq)]
enum ParserError {
    UnexpectedToken,
    EOFWhileParsingExpression,
    LexerError(LexerError),
}

pub struct Parser<T> {
    lexer: Lexer<T>,
    cur_evt: Option<LexerEvent>,
}

impl<T: Iterator<Item=char>> Parser<T> {
    pub fn new(src: T) -> Parser<T> {
        Parser {
            lexer: Lexer::new(src),
            cur_evt: None,
        }
    }

    fn bump(&mut self) {
        self.cur_evt = self.lexer.next()
    }

    fn parse(&mut self) -> Result<Sexp, ParserError> {
        self.bump();
        let result = self.parse_sexp();
        if result.is_ok() {
            self.bump()
        }
        match self.cur_evt {
            None => {
                result
            },
            Some(LexerEvent::Token(_)) => {
                Err(ParserError::UnexpectedToken)
            },
            Some(LexerEvent::Error(e)) => {
                Err(ParserError::LexerError(e))
            },
        }
    }

    fn parse_sexp(&mut self) -> Result<Sexp, ParserError> {
        match self.cur_evt {
            Some(LexerEvent::Token(Token::Number(n))) => {
                Ok(Sexp::Atom(Atom::Number(n)))
            },
            Some(LexerEvent::Token(Token::String(ref s))) => {
                Ok(Sexp::Atom(Atom::String(s.clone())))
            },
            Some(LexerEvent::Token(Token::Symbol(ref s))) => {
                Ok(Sexp::Atom(Atom::Symbol(s.clone())))
            },
            Some(LexerEvent::Token(Token::ListStart)) => {
                self.parse_list()
            },
            Some(LexerEvent::Token(Token::ListEnd)) => {
                Err(ParserError::UnexpectedToken)
            },
            Some(LexerEvent::Error(e)) => {
                Err(ParserError::LexerError(e))
            },
            None => {
                Err(ParserError::EOFWhileParsingExpression)
            }
        }
    }

    fn parse_list(&mut self) -> Result<Sexp, ParserError> {
        let mut list = Vec::new();
        loop {
            self.bump();
            if self.cur_evt == Some(LexerEvent::Token(Token::ListEnd)) {
                return Ok(Sexp::List(list))
            }
            match self.parse_sexp() {
                Ok(s) => {
                    list.push(s);
                },
                e @ Err(_) => {
                    return e
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common::Atom::{Number, Symbol};
    use common::Sexp::{List, Atom};
    use super::Parser;

    #[test]
    fn test_parse_list_expression() {
        let expected_result = List(vec![Atom(Symbol("def".to_string())),
                                        Atom(Symbol("a".to_string())),
                                        Atom(Number(1_f64))]);
        let mut parser = Parser::new("(def a 1)".chars());
        let actual_result = parser.parse().ok().unwrap();
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_parse_nested_list_expressions() {
        let expected_result =  List(vec![Atom(Symbol("def".to_string())),
                                         Atom(Symbol("a".to_string())),
                                         List(vec![Atom(Symbol("+".to_string())),
                                                   Atom(Number(1_f64)),
                                                   Atom(Number(2_f64))])]);
        let mut parser = Parser::new("(def a (+ 1 2))".chars());
        let actual_result = parser.parse().ok().unwrap();
        assert_eq!(expected_result, actual_result);
    }
}
