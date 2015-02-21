use std::fmt::{Display, Formatter, Result};
use common::{Atom, Sexp};

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Atom::Number(n) => {
                write!(f, "{}", n)
            },
            Atom::Symbol(ref s) => {
                write!(f, "{}", s)
            },
            Atom::String(ref s) => {
                write!(f, r#""{}""#, s)
            },
        }
    }
}

impl Display for Vec<Sexp> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut a = "(".to_string();
        let i_last = self.len() - 1;
        for (i, s) in self.iter().enumerate() {
            if i < i_last {
                a.push_str(&format!("{} ", s))
            } else {
                a.push_str(&format!("{}", s))
            }
        }
        a.push_str(")");
        write!(f, "{}", a)
    }
}

impl Display for Sexp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Sexp::Atom(ref atom) => {
                write!(f, "{}", atom)
            },
            Sexp::List(ref l) => {
                write!(f, "{}", l)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common::Sexp::{List, Atom};
    use common::Atom::{Symbol, Number};

    #[test]
    fn test_format_list_with_nested_list_and_atoms() {
        let actual_input = List(vec![Atom(Symbol("def".to_string())),
                                     Atom(Symbol("a".to_string())),
                                     List(vec![Atom(Symbol("+".to_string())),
                                               Atom(Number(1_f64)),
                                               Atom(Number(2_f64))])]);
        let actual_result = format!("{}", actual_input);
        let expected_result = "(def a (+ 1 2))";
        assert_eq!(expected_result, actual_result);
    }
}
