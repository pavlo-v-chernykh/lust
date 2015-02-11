use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Atom {
    Number(f64),
    Symbol(String)
}

#[derive(Debug, PartialEq)]
enum ParseAtomError {
    IncorrectSymbolName
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Atom::Number(n) => {
                write!(f, "{}", n)
            },
            Atom::Symbol(ref name) => {
                write!(f, "'{}", name)
            }
        }
    }
}

impl FromStr for Atom {
    type Err = ParseAtomError;

    fn from_str(s: &str) -> Result<Atom, ParseAtomError> {
        match s.parse::<f64>() {
            Ok(f) => {
                Ok(Atom::Number(f))
            },
            Err(..) => {
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

#[cfg_attr(test, allow(dead_code))]
fn main() {
}

#[cfg(test)]
mod tests {
    use super::Atom::{self, Number, Symbol};
    use super::ParseAtomError::IncorrectSymbolName;
    use super::tokenize;

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
}
