use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Number(n) => {
                write!(f, "{}", n)
            },
            Expr::Symbol(ref s) => {
                write!(f, "{}", s)
            },
            Expr::String(ref s) => {
                write!(f, r#""{}""#, s)
            },
            Expr::List(ref l) => {
                write!(f, "({})", l)
            }
        }
    }
}

impl fmt::Display for Vec<Expr> {
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
    #[test]
    fn test_format_list_with_nested_list_and_atoms() {
        let actual_input = e_list![e_symbol!("def"),
                                   e_symbol!("a"),
                                   e_list![e_symbol!("+"),
                                           e_number!(1_f64),
                                           e_number!(2_f64)]];
        let actual_result = format!("{}", actual_input);
        let expected_result = "(def a (+ 1 2))";
        assert_eq!(expected_result, actual_result);
    }
}
