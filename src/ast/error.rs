use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum ExpandErrorCode {
    UnknownError
}

#[derive(Debug, PartialEq)]
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
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::ExpandError;
    use super::ExpandErrorCode::*;

    #[test]
    fn test_descriptions_for_error_codes() {
        let err = ExpandError(UnknownError);
        assert_eq!("Unknown expanstion error", format!("{}", err));
    }
}
