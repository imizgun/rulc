use crate::core::evaluator::evaluator_result::Value;
use std::fmt::Display;

pub enum ReplOutput {
    Value(Value),
    Message(String),
    FuncPoints { points: Vec<(f64, f64)> },
    IntersectionPoints { points: Vec<(f64, f64)> },
    ClearPlots,
    ClearHistory,
    ClearAll
}

impl Display for ReplOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReplOutput::Value(v) => write!(f, "{}", v),
            ReplOutput::Message(m) => write!(f, "{}", m),
            ReplOutput::FuncPoints { points } => write!(f, "points"),
            ReplOutput::ClearPlots => write!(f, "plots were cleared"),
            ReplOutput::IntersectionPoints { points } => write!(f, "intersection points"),
            ReplOutput::ClearHistory => write!(f, "history cleared"),
            ReplOutput::ClearAll => write!(f, "everything cleared"),
        }
    }
}
