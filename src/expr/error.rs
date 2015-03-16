use std::fmt;

#[derive(Debug, PartialEq)]
pub enum EvalError {
    ResolveError(String),
    UnknownError,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &EvalError::ResolveError(ref name) => {
                write!(f, r#"Unable to resolve symbol "{}" in this context"#, name)
            },
            &EvalError::UnknownError => {
                write!(f, "Unknown evaluation error")
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EvalError;

    #[test]
    fn test_descriptions_for_error_codes() {
        let err = EvalError::UnknownError;
        assert_eq!("Unknown evaluation error", format!("{}", err));
        let err = EvalError::ResolveError("name".to_string());
        assert_eq!(r#"Unable to resolve symbol "name" in this context"#, format!("{}", err));
    }
}
