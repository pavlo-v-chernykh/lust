mod error;

use self::error::ParserError;
use ast::Expr;
use lexer::{Token, Lexer, LexerResult};

pub type ParserResult = Result<Expr, ParserError>;

pub struct Parser<I: Iterator> {
    lexer: Lexer<I>,
    token: Option<LexerResult>,
}

impl<I: Iterator<Item=char>> Iterator for Parser<I> {
    type Item = ParserResult;

    fn next(&mut self) -> Option<ParserResult> {
        self.parse()
    }
}

impl<I: Iterator<Item=char>> Parser<I> {
    pub fn new(src: I) -> Parser<I> {
        Parser {
            lexer: Lexer::new(src),
            token: None,
        }
    }

    fn bump(&mut self) {
        self.token = self.lexer.next()
    }

    fn parse(&mut self) -> Option<ParserResult> {
        self.bump();
        if self.token.is_some() {
            Some(self.parse_expr())
        } else {
            None
        }
    }

    fn parse_expr(&mut self) -> ParserResult {
        match self.token {
            Some(Ok(Token::Number { val, .. })) => {
                Ok(Expr::Number(val))
            },
            Some(Ok(Token::String { ref val, .. })) => {
                Ok(Expr::String(val.clone()))
            },
            Some(Ok(Token::Symbol { ref val, .. })) => {
                Ok(Expr::Symbol(val.clone()))
            },
            Some(Ok(Token::Keyword { ref val, .. })) => {
                Ok(Expr::Keyword(val.clone()))
            },
            Some(Ok(Token::ListStart { .. })) => {
                self.parse_list()
            },
            Some(Ok(Token::VecStart { .. })) => {
                self.parse_vec()
            },
            Some(Ok(Token::Quote { .. })) => {
                self.parse_quoted()
            },
            Some(Ok(Token::Unquote { .. })) => {
                self.parse_unquoted()
            },
            Some(Ok(Token::UnquoteSplicing { .. })) => {
                self.parse_unquoted_splicing()
            },
            Some(Ok(ref t @ Token::ListEnd { .. })) |
            Some(Ok(ref t @ Token::VecEnd { .. })) => {
                Err(ParserError::UnexpectedToken(t.clone()))
            },
            Some(Err(e)) => {
                Err(ParserError::LexerError(e))
            },
            None => {
                Err(ParserError::UnexpectedEndOfInput)
            }
        }
    }

    fn parse_list(&mut self) -> ParserResult {
        let mut list = Vec::new();
        loop {
            self.bump();
            if let Some(Ok(Token::ListEnd { .. })) = self.token {
                return Ok(Expr::List(list))
            }
            list.push(try!(self.parse_expr()))
        }
    }

    fn parse_vec(&mut self) -> ParserResult {
        let mut vec = Vec::new();
        loop {
            self.bump();
            if let Some(Ok(Token::VecEnd { .. })) = self.token {
                return Ok(Expr::Vec(vec))
            }
            vec.push(try!(self.parse_expr()))
        }
    }

    fn parse_quoted(&mut self) -> ParserResult {
        self.bump();
        Ok(e_list![e_symbol!["quote"], try!(self.parse_expr())])
    }

    fn parse_unquoted(&mut self) -> ParserResult {
        self.bump();
        Ok(e_list![e_symbol!["unquote"], try!(self.parse_expr())])
    }

    fn parse_unquoted_splicing(&mut self) -> ParserResult {
        self.bump();
        Ok(e_list![e_symbol!["unquote-splicing"], try!(self.parse_expr())])
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn test_parse_list_expression() {
        let expected_result = e_list![e_symbol!("def"),
                                      e_symbol!("a"),
                                      e_number!(1_f64)];
        let mut parser = Parser::new("(def a 1)".chars());
        let actual_result = parser.next().unwrap().ok().unwrap();
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_parse_nested_list_expressions() {
        let expected_result =  e_list![e_symbol!("def"),
                                       e_symbol!("a"),
                                       e_list![e_symbol!("+"),
                                               e_number!(1_f64),
                                               e_number!(2_f64)]];
        let mut parser = Parser::new("(def a (+ 1 2))".chars());
        let actual_result = parser.next().unwrap().ok().unwrap();
        assert_eq!(expected_result, actual_result);
    }
}
