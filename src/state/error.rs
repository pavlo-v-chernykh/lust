use std::fmt;
use ast::Expr;

#[derive(Debug, PartialEq)]
pub enum EvalError {
    ResolveError(String),
    DispatchError(Expr),
    IncorrectTypeOfArgumentError(Expr),
    IncorrectNumberOfArgumentsError(Expr),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvalError::ResolveError(ref name) => {
                write!(f, r#"Unable to resolve symbol "{}""#, name)
            },
            EvalError::DispatchError(ref expr) => {
                write!(f, r#"Unable to dispatch expression "{}""#, expr)
            },
            EvalError::IncorrectTypeOfArgumentError(ref expr) => {
                write!(f, r#"Incorrect type of argument "{}""#, expr)
            },
            EvalError::IncorrectNumberOfArgumentsError(ref expr) => {
                write!(f, r#"Incorrect number of arguments {}"#, expr)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EvalError;

    #[test]
    fn test_descriptions_for_error_codes() {
        let err = EvalError::ResolveError("name".to_string());
        assert_eq!(r#"Unable to resolve symbol "name""#, format!("{}", err));
        let err = EvalError::DispatchError(e_list![e_symbol!["def"],
                                                   e_symbol!["a"],
                                                   e_number![1.]]);
        assert_eq!(r#"Unable to dispatch expression "(def a 1)""#,
                   format!("{}", err));
        let err = EvalError::IncorrectTypeOfArgumentError(e_symbol!["a"]);
        assert_eq!(r#"Incorrect type of argument "a""#, format!("{}", err));
        let err = EvalError::IncorrectNumberOfArgumentsError(e_call!["+",]);
        assert_eq!(r#"Incorrect number of arguments (+)"#, format!("{}", err));
    }
}
