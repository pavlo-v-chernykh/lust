use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum EvalErrorCode {
    UnknownError
}

#[derive(Debug, PartialEq)]
pub struct EvalError(EvalErrorCode);

impl EvalError {
    pub fn new(code: EvalErrorCode) -> EvalError {
        EvalError(code)
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl error::Error for EvalError {
    fn description(&self) -> &str {
        match self {
            &EvalError(EvalErrorCode::UnknownError) => {
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
    use super::EvalErrorCode::*;

    #[test]
    fn test_descriptions_for_error_codes() {
        let err = EvalError(UnknownError);
        assert_eq!("Unknown evaluation error", format!("{}", err));
    }
}
