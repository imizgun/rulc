use std::string::ParseError;
use crate::core::evaluator::evaluation_error::EvaluationError;

#[derive(Debug)]
pub enum RuntimeError {
    ParseError(ParseError),
    EvalError(EvaluationError)
}