use common::{Atom, Sexp};

#[derive(Debug, PartialEq)]
enum ParserState {
    StartRead,
    OpenList,
    AtomRead(Atom),
    CloseList,
    EndRead,
}

#[derive(Debug, PartialEq)]
enum ParserError {
}

fn parse(tokens: Vec<String>) -> Result<Vec<Sexp>, ParserError> {
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
        let expected_result = vec![];
        assert_eq!(expected_result, parse(vec![]).ok().unwrap())
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
        let actual_result = parse(actual_input).ok().unwrap();
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
        let actual_result = parse(actual_input).ok().unwrap();
        assert_eq!(expected_result, actual_result);
    }
}
