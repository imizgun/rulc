use std::fmt::{Display, Formatter};
use crate::core::error_display::Located;
use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::parser::parse_error::ParseError;

#[derive(Debug)]
pub enum RuntimeError {
    ParseError(Located<ParseError>),
    EvalError(Located<EvaluationError>),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::ParseError(e) => write!(f, "{}", e),
            RuntimeError::EvalError(e) => write!(f, "{}", e),
        }
    }
}

impl From<Located<ParseError>> for RuntimeError {
    fn from(e: Located<ParseError>) -> Self {
        RuntimeError::ParseError(e)
    }
}

impl From<Located<EvaluationError>> for RuntimeError {
    fn from(e: Located<EvaluationError>) -> Self {
        RuntimeError::EvalError(e)
    }
}