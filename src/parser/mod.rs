mod error;
#[cfg(test)]
mod tests;

use ast::Node;
use ast::nodes::Symbol;
use lexer::{Token, Lexer, LexerResult};

pub use self::error::ParserError;
pub type ParserResult = Result<Node, ParserError>;

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
                Ok(n_number![val])
            },
            Some(Ok(Token::String { ref val, .. })) => {
                Ok(n_string![val.clone()])
            },
            Some(Ok(Token::Symbol { ref ns, ref name, .. })) => {
                Ok(n_symbol![ns.clone(), name.clone()])
            },
            Some(Ok(Token::Keyword { ref ns, ref name, .. })) => {
                Ok(n_keyword![ns.clone(), name.clone()])
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
            Some(Ok(Token::SyntaxQuote { .. })) => {
                self.parse_syntax_quote()
            },
            Some(Ok(ref t @ Token::ListEnd { .. })) |
            Some(Ok(ref t @ Token::VecEnd { .. })) => {
                Err(ParserError::UnexpectedToken(t.clone()))
            },
            Some(Err(ref e)) => {
                Err(ParserError::LexerError(e.clone()))
            },
            None => {
                Err(ParserError::UnexpectedEndOfInput)
            }
        }
    }

    fn parse_list(&mut self) -> ParserResult {
        let mut list = vec![];
        loop {
            self.bump();
            if let Some(Ok(Token::ListEnd { .. })) = self.token {
                return Ok(n_list![list])
            }
            list.push(try!(self.parse_expr()))
        }
    }

    fn parse_vec(&mut self) -> ParserResult {
        let mut vec = Vec::new();
        loop {
            self.bump();
            if let Some(Ok(Token::VecEnd { .. })) = self.token {
                return Ok(Node::Vec(vec))
            }
            vec.push(try!(self.parse_expr()))
        }
    }

    fn parse_quoted(&mut self) -> ParserResult {
        self.bump();
        Ok(n_list![vec![n_symbol!["quote"], try!(self.parse_expr())]])
    }

    fn parse_unquoted(&mut self) -> ParserResult {
        self.bump();
        Ok(n_list![vec![n_symbol!["unquote"], try!(self.parse_expr())]])
    }

    fn parse_unquoted_splicing(&mut self) -> ParserResult {
        self.bump();
        Ok(n_list![vec![n_symbol!["unquote-splicing"], try!(self.parse_expr())]])
    }

    fn parse_syntax_quote(&mut self) -> ParserResult {
        self.bump();
        Ok(n_list![vec![n_symbol!["syntax-quote"], try!(self.parse_expr())]])
    }
}
