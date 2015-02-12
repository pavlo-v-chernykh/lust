use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Atom {
    Number(f64),
    Symbol(String),
    Nil
}

#[derive(Debug, PartialEq, Clone)]
enum Sexp {
    Atom(Atom),
    List(Vec<Sexp>)
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

fn tokenize(s: &str) -> Vec<String> {
    s.replace("("," ( ")
        .replace(")", " ) ")
        .replace("\n", " ")
        .split(' ')
        .filter(|s| { !s.is_empty() })
        .map(|s| { s.to_string() })
        .collect()
}

#[derive(Debug, PartialEq)]
enum ReadState {
    StartRead,
    OpenList,
    AtomRead(Atom),
    CloseList,
    EndRead,
}

#[derive(Debug, PartialEq)]
enum ReadError {
}

fn read(tokens: Vec<String>) -> Result<Vec<Sexp>, ReadError> {
    let mut state = ReadState::StartRead;
    let mut iter = tokens.iter();
    let mut result = vec![];
    loop {
        match state {
            ReadState::StartRead => {
                match iter.next() {
                    Some(s) if *s == "(" => {
                        state = ReadState::OpenList;
                        result.push(Sexp::List(vec![]));
                    },
                    _ => {
                        state = ReadState::EndRead;
                    }
                }
            },
            ReadState::OpenList => {
                match iter.next() {
                    Some(s) if *s == ")" => {
                        state = ReadState::CloseList;
                    },
                    Some(s) => {
                        state = ReadState::AtomRead(s.parse::<Atom>().ok().unwrap());
                    },
                    _ => {
                        state = ReadState::EndRead;
                    }
                }
            },
            ReadState::CloseList => {
                match iter.next() {
                    Some(s) if *s == "(" => {
                        state = ReadState::OpenList;
                        result.push(Sexp::List(vec![]));
                    },
                    _ => {
                        state = ReadState::EndRead;
                    }
                }
            },
            ReadState::AtomRead(atom) => {
                if let Some(Sexp::List(mut current_list)) = result.pop() {
                    current_list.push(Sexp::Atom(atom));
                    result.push(Sexp::List(current_list));
                }
                match iter.next() {
                    Some(s) if *s == ")" => {
                        state = ReadState::CloseList;
                    },
                    Some(s) => {
                        state = ReadState::AtomRead(s.parse::<Atom>().ok().unwrap());
                    },
                    _ => {
                        state = ReadState::EndRead;
                    }
                }
            },
            ReadState::EndRead => {
                return Ok(result)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum EvalError{
    EvalError
}

fn eval(s: Sexp, e: &HashMap<String, Sexp>) -> Result<Sexp, EvalError> {
    match s {
        Sexp::Atom(Atom::Number(_)) => {
            Ok(s)
        },
        Sexp::Atom(Atom::Symbol(ref name)) => {
            if let Some(s) = e.get(name) {
                Ok(s.clone())
            } else {
                Err(EvalError::EvalError)
            }
        }
        _ => {
            Err(EvalError::EvalError)
        }
    }
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::Atom::{self, Number, Symbol, Nil};
    use super::Sexp;
    use super::ParseAtomError::IncorrectSymbolName;
    use super::EvalError::EvalError;
    use super::{tokenize, read, eval};

    #[test]
    fn test_parse_integer() {
        assert_eq!(Number(64f64), "64".parse::<Atom>().ok().unwrap())
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(Number(64.5), "64.5".parse::<Atom>().ok().unwrap())
    }

    #[test]
    fn test_parse_symbol() {
        assert_eq!(Symbol("name".to_string()), "name".parse::<Atom>().ok().unwrap())
    }

    #[test]
    fn test_parse_incorrect_symbol_starting_with_digit() {
        assert_eq!(IncorrectSymbolName, "6name".parse::<Atom>().err().unwrap())
    }

    #[test]
    fn test_tokenize_dense_expression() {
        let expected_result = ["(", "def", "a", "1", ")"]
                                .iter()
                                .map(|s| { s.to_string() })
                                .collect();
        assert_eq!(expected_result, tokenize("(def a 1)"))
    }

    #[test]
    fn test_tokenize_sparse_expression() {
        let expected_result = ["(", "def", "a", "1", ")"]
                                .iter()
                                .map(|s| { s.to_string() })
                                .collect();
        assert_eq!(expected_result, tokenize(" ( \n def a\n1)   \n"))
    }

    #[test]
    fn test_read_empty() {
        let expected_result = vec![];
        assert_eq!(expected_result, read(tokenize("")).ok().unwrap())
    }

    #[test]
    fn test_read_single_expression() {
        let expected_result =  vec![Sexp::List(vec![Sexp::Atom(Symbol("def".to_string())),
                                                    Sexp::Atom(Symbol("a".to_string())),
                                                    Sexp::Atom(Number(1f64))])];
        assert_eq!(expected_result, read(tokenize("(def a 1)")).ok().unwrap())
    }

    #[test]
    fn test_read_multiple_expression() {
        let expected_result =  vec![Sexp::List(vec![Sexp::Atom(Symbol("def".to_string())),
                                                    Sexp::Atom(Symbol("a".to_string())),
                                                    Sexp::Atom(Number(1f64))]),
                                    Sexp::List(vec![Sexp::Atom(Symbol("def".to_string())),
                                                    Sexp::Atom(Symbol("b".to_string())),
                                                    Sexp::Atom(Number(2f64))]),
                                    Sexp::List(vec![Sexp::Atom(Symbol("+".to_string())),
                                                    Sexp::Atom(Symbol("a".to_string())),
                                                    Sexp::Atom(Symbol("b".to_string()))])];
        assert_eq!(expected_result, read(tokenize("(def a 1)(def b 2)(+ a b)")).ok().unwrap())
    }

    #[test]
    fn test_eval_atom_number() {
        let number = 10f64;
        let expected_result = Sexp::Atom(Number(number));
        let actual_result = eval(Sexp::Atom(Number(number)), &HashMap::new());
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_number() {
        let num = 10f64;
        let mut env = HashMap::new();
        env.insert("a".to_string(), Sexp::Atom(Number(num)));
        let expected_result = Sexp::Atom(Number(num));
        let actual_result = eval(Sexp::Atom(Symbol("a".to_string())), &env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_nil() {
        let mut env = HashMap::new();
        env.insert("a".to_string(), Sexp::Atom(Nil));
        let expected_result = Sexp::Atom(Nil);
        let actual_result = eval(Sexp::Atom(Symbol("a".to_string())), &env);
        assert_eq!(expected_result, actual_result.ok().unwrap());
    }

    #[test]
    fn test_eval_atom_symbol_to_non_value() {
        let env = HashMap::new();
        let expected_result = EvalError;
        let actual_result = eval(Sexp::Atom(Symbol("a".to_string())), &env);
        assert_eq!(expected_result, actual_result.err().unwrap());
    }
}
