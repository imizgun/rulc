use std::fmt::Display;
use crate::core::evaluator::evaluator_result::Value;

pub enum ReplOutput {
    Value(Value),
    Message(String),
    FuncPoints{ points: Vec<(f64, f64)> },
}

impl Display for ReplOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReplOutput::Value(v) => write!(f, "{}", v),
            ReplOutput::Message(m) => write!(f, "{}", m),
            ReplOutput::FuncPoints{ points } => write!(f, "points"),
        }
    }
}