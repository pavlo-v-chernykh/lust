use std::fmt;
use ast::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    Number(f64),
    Bool(bool),
    String(String),
    Symbol(String),
    List(Vec<Val>),
    Fn {
        params: Vec<Expr>,
        body: Vec<Expr>,
    },
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Val::Number(n) => {
                write!(f, "{}", n)
            },
            Val::Bool(b) => {
                write!(f, "{}", b)
            },
            Val::String(ref s) => {
                write!(f, "{}", s)
            },
            Val::Symbol(ref s) => {
                write!(f, "{}", s)
            },
            Val::List(ref l) => {
                write!(f, "({})", l)
            },
            Val::Fn { ref params, ref body } => {
                write!(f, "(fn ({}) {})", params, body)
            }
        }
    }
}

impl fmt::Display for Vec<Val> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut a = String::new();
        let i_last = self.len() - 1;
        for (i, s) in self.iter().enumerate() {
            if i < i_last {
                a.push_str(&format!("{} ", s))
            } else {
                a.push_str(&format!("{}", s))
            }
        }
        write!(f, "{}", a)
    }
}

#[cfg(test)]
mod tests {
    use super::Val;

    #[test]
    fn test_format_list_with_nested_list_and_atoms() {
        let actual_input = Val::List(vec![Val::Symbol("def".to_string()),
                                          Val::Symbol("a".to_string()),
                                          Val::List(vec![Val::Symbol("+".to_string()),
                                                         Val::Number(1_f64),
                                                         Val::Number(2_f64)])]);
        let actual_result = format!("{}", actual_input);
        let expected_result = "(def a (+ 1 2))";
        assert_eq!(expected_result, actual_result);
    }
}
