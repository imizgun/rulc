use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum EvaluationError {
    InvalidTokenPlace(String),
    MissingOperand,
    ArithmeticError(String)
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError::InvalidTokenPlace(msg) => write!(f, "this token can't be placed here: '{}'", msg),
            EvaluationError::MissingOperand => write!(f, "missing operand here ^"),
            EvaluationError::ArithmeticError(msg) => write!(f, "arithmetic error: '{}'", msg),
        }
    }
}