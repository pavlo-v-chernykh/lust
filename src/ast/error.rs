use std::{error, fmt};
use expr::EvalError;

#[derive(Debug, PartialEq, Copy)]
pub enum ExpandErrorCode {
    UnknownError,
    EvalError(EvalError)
}

#[derive(Debug, PartialEq, Copy)]
pub struct ExpandError(ExpandErrorCode);

impl ExpandError {
    pub fn new(code: ExpandErrorCode) -> ExpandError {
        ExpandError(code)
    }
}

impl fmt::Display for ExpandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl error::Error for ExpandError {
    fn description(&self) -> &str {
        match self {
            &ExpandError(ExpandErrorCode::UnknownError) => {
                "Unknown expanstion error"
            },
            &ExpandError(ExpandErrorCode::EvalError(ref e)) => {
                e.description()
            }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &ExpandError(ExpandErrorCode::EvalError(ref e)) => {
                Some(e)
            },
            _ => {
                None
            }
        }
    }
}

impl error::FromError<EvalError> for ExpandError {
    fn from_error(err: EvalError) -> Self {
        ExpandError::new(ExpandErrorCode::EvalError(err))
    }
}

#[cfg(test)]
mod tests {
    use std::error::FromError;
    use super::{ExpandError, ExpandErrorCode};
    use expr::{EvalError, EvalErrorCode};

    #[test]
    fn test_descriptions_for_error_codes() {
        let err = ExpandError::new(ExpandErrorCode::UnknownError);
        assert_eq!("Unknown expanstion error", format!("{}", err));
        let eval_error = EvalError::new(EvalErrorCode::UnknownError);
        let err = ExpandError::new(ExpandErrorCode::EvalError(eval_error));
        assert_eq!("Unknown evaluation error", format!("{}", err));

    }

    #[test]
    fn test_expand_error_from_eval_error() {
        let eval_error = EvalError::new(EvalErrorCode::UnknownError);
        let expected_result = ExpandError::new(ExpandErrorCode::EvalError(eval_error));
        assert_eq!(expected_result, FromError::from_error(eval_error));
    }
}
