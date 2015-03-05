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
