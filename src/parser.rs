use std::str::FromStr;
use common::{Atom, Sexp};
use lexer::{Lexer, LexerEvent, LexerError};

#[derive(Debug, PartialEq)]
enum ParserError {
    UnexpectedToken,
    LexerError(LexerError),
}

struct Parser<T> {
    lexer: Lexer<T>,
    cur_evt: Option<LexerEvent>,
}

impl<T: Iterator<Item=char>> Parser<T> {
    fn new(src: T) -> Parser<T> {
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
        Ok(Sexp::Atom(Atom::Nil))
    }
}


#[derive(Debug, PartialEq)]
enum ParserState {
    StartRead,
    OpenList,
    AtomRead(Atom),
    CloseList,
    EndRead,
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

fn parse(tokens: &Vec<String>) -> Result<Vec<Sexp>, ParserError> {
    let mut state = ParserState::StartRead;
    let mut iter = tokens.iter();
    let mut result = vec![];
    loop {
        match state {
            ParserState::StartRead => {
                match iter.next() {
                    Some(s) if *s == "(" => {
                        state = ParserState::OpenList;
                        result.push(Sexp::List(vec![]));
                    },
                    _ => {
                        state = ParserState::EndRead;
                    }
                }
            },
            ParserState::OpenList => {
                match iter.next() {
                    Some(s) if *s == ")" => {
                        state = ParserState::CloseList;
                    },
                    Some(s) => {
                        state = ParserState::AtomRead(s.parse::<Atom>().ok().unwrap());
                    },
                    _ => {
                        state = ParserState::EndRead;
                    }
                }
            },
            ParserState::CloseList => {
                match iter.next() {
                    Some(s) if *s == "(" => {
                        state = ParserState::OpenList;
                        result.push(Sexp::List(vec![]));
                    },
                    _ => {
                        state = ParserState::EndRead;
                    }
                }
            },
            ParserState::AtomRead(atom) => {
                if let Some(Sexp::List(mut current_list)) = result.pop() {
                    current_list.push(Sexp::Atom(atom));
                    result.push(Sexp::List(current_list));
                }
                match iter.next() {
                    Some(s) if *s == ")" => {
                        state = ParserState::CloseList;
                    },
                    Some(s) => {
                        state = ParserState::AtomRead(s.parse::<Atom>().ok().unwrap());
                    },
                    _ => {
                        state = ParserState::EndRead;
                    }
                }
            },
            ParserState::EndRead => {
                return Ok(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common::Atom::{Number, Symbol};
    use common::Sexp::{List, Atom};
    use super::parse;


    #[test]
    fn test_parse_empty() {
        assert_eq!(vec![], parse(&vec![]).ok().unwrap())
    }

    #[test]
    fn test_parse_single_expression() {
        let expected_result =  vec![List(vec![Atom(Symbol("def".to_string())),
                                              Atom(Symbol("a".to_string())),
                                              Atom(Number(1f64))])];
        let actual_input = ["(", "def", "a", "1", ")"]
                                .iter()
                                .map(|s| { s.to_string() })
                                .collect();
        let actual_result = parse(&actual_input).ok().unwrap();
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn test_parse_multiple_expression() {
        let expected_result =  vec![List(vec![Atom(Symbol("def".to_string())),
                                              Atom(Symbol("a".to_string())),
                                              Atom(Number(1f64))]),
                                    List(vec![Atom(Symbol("def".to_string())),
                                              Atom(Symbol("b".to_string())),
                                              Atom(Number(2f64))]),
                                    List(vec![Atom(Symbol("+".to_string())),
                                              Atom(Symbol("a".to_string())),
                                              Atom(Symbol("b".to_string()))])];
        let actual_input = ["(", "def", "a", "1", ")",
                            "(", "def", "b", "2", ")",
                            "(", "+", "a", "b", ")"]
                                .iter()
                                .map(|s| { s.to_string() })
                                .collect();
        let actual_result = parse(&actual_input).ok().unwrap();
        assert_eq!(expected_result, actual_result);
    }
}
