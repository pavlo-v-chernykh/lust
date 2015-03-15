use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum EvalError {
    ResolveError(String),
    UnknownError,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &EvalError::ResolveError(ref name) => {
                write!(f, r#"{} "{}" in this context"#, error::Error::description(self), name)
            },
            &EvalError::UnknownError => {
                write!(f, "{}", error::Error::description(self))
            },
        }
    }
}

impl error::Error for EvalError {
    fn description(&self) -> &str {
        match self {
            &EvalError::ResolveError(_) => {
                "Unnable to resolve symbol"
            },
            &EvalError::UnknownError => {
                "Unknown evaluation error"
            },
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
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
        assert_eq!(r#"Unnable to resolve symbol "name" in this context"#, format!("{}", err));
    }
}
