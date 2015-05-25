use std::fmt;
use std::convert::From;
use std::io::Error as IoError;
use std::error::Error;
use ast::Node;
use parser::ParserError;

#[derive(Debug, PartialEq)]
pub enum EvalError {
    ResolveError(String),
    DispatchError(Node),
    IncorrectTypeOfArgumentError(Node),
    IncorrectNumberOfArgumentsError(Node),
    IoError(String),
    ParserError(ParserError),
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
            EvalError::IoError(ref e) => {
                write!(f, r#"{}"#, e)
            },
            EvalError::ParserError(ref e) => {
                write!(f, r#"{}"#, e)
            },
        }
    }
}

impl From<IoError> for EvalError {
    fn from(e: IoError) -> EvalError {
        EvalError::IoError(e.description().to_string())
    }
}

impl From<ParserError> for EvalError {
    fn from(e: ParserError) -> EvalError {
        EvalError::ParserError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::EvalError;

    #[test]
    fn test_descriptions_for_error_codes() {
        let err = EvalError::ResolveError("name".to_string());
        assert_eq!(r#"Unable to resolve symbol "name""#, format!("{}", err));
        let err = EvalError::DispatchError(n_list![n_symbol!["def"],
                                                   n_symbol!["a"],
                                                   n_number![1.]]);
        assert_eq!(r#"Unable to dispatch expression "(def a 1)""#,
                   format!("{}", err));
        let err = EvalError::IncorrectTypeOfArgumentError(n_symbol!["a"]);
        assert_eq!(r#"Incorrect type of argument "a""#, format!("{}", err));
        let err = EvalError::IncorrectNumberOfArgumentsError(n_call!["+", vec![]]);
        assert_eq!(r#"Incorrect number of arguments (+)"#, format!("{}", err));
    }
}
