use ast::Expr;
use token::Token;
use lexer::{Lexer, LexerResult, LexerError};

#[derive(Debug, PartialEq)]
enum ParserError {
    UnexpectedToken,
    EOFWhileParsingExpression,
    LexerError(LexerError),
}

pub struct Parser<I: Iterator> {
    lexer: Lexer<I>,
    token: Option<LexerResult>,
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

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.bump();
        let result = self.parse_expr();
        if result.is_ok() {
            self.bump()
        }
        match self.token {
            None => {
                result
            },
            Some(Ok(_)) => {
                Err(ParserError::UnexpectedToken)
            },
            Some(Err(e)) => {
                Err(ParserError::LexerError(e))
            },
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        match self.token {
            Some(Ok(Token::Number(n))) => {
                Ok(Expr::Number(n))
            },
            Some(Ok(Token::String(ref s))) => {
                Ok(Expr::String(s.clone()))
            },
            Some(Ok(Token::Symbol(ref s))) => {
                Ok(Expr::Symbol(s.clone()))
            },
            Some(Ok(Token::ListStart)) => {
                self.parse_list()
            },
            Some(Ok(Token::ListEnd)) => {
                Err(ParserError::UnexpectedToken)
            },
            Some(Err(e)) => {
                Err(ParserError::LexerError(e))
            },
            None => {
                Err(ParserError::EOFWhileParsingExpression)
            }
        }
    }

    fn parse_list(&mut self) -> Result<Expr, ParserError> {
        let mut list = Vec::new();
        loop {
            self.bump();
            if self.token == Some(Ok(Token::ListEnd)) {
                return Ok(Expr::List(list))
            }
            match self.parse_expr() {
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
    use ast::Expr;
    use super::Parser;

    #[test]
    fn test_parse_list_expression() {
        let expected_result = Expr::List(vec![Expr::Symbol("def".to_string()),
                                              Expr::Symbol("a".to_string()),
                                              Expr::Number(1_f64)]);
        let mut parser = Parser::new("(def a 1)".chars());
        let actual_result = parser.parse().ok().unwrap();
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_parse_nested_list_expressions() {
        let expected_result =  Expr::List(vec![Expr::Symbol("def".to_string()),
                                               Expr::Symbol("a".to_string()),
                                               Expr::List(vec![Expr::Symbol("+".to_string()),
                                                               Expr::Number(1_f64),
                                                               Expr::Number(2_f64)])]);
        let mut parser = Parser::new("(def a (+ 1 2))".chars());
        let actual_result = parser.parse().ok().unwrap();
        assert_eq!(expected_result, actual_result);
    }
}
