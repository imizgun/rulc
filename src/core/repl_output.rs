use crate::core::evaluator::evaluator_result::Value;
use std::fmt::Display;

pub enum ReplOutput {
    Value(Value),
    Message(String),
    FuncPoints { points: Vec<(f64, f64)> },
    IntersectionPoints { points: Vec<(f64, f64)> },
    Clear(ReplClearOutput),
}

pub enum ReplClearOutput {
    ClearAll,
    ClearHistory,
    ClearPlots,
    ClearMemory,
    ClearVariable(String),
}

impl Display for ReplOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReplOutput::Value(v) => write!(f, "{}", v),
            ReplOutput::Message(m) => write!(f, "{}", m),
            ReplOutput::FuncPoints { points } => write!(f, "points"),
            ReplOutput::IntersectionPoints { points } => write!(f, "intersection points"),
            ReplOutput::Clear(com) => match com {
                ReplClearOutput::ClearAll => write!(f, "everything cleared"),
                ReplClearOutput::ClearHistory => write!(f, "history cleared"),
                ReplClearOutput::ClearPlots => write!(f, "plots cleared"),
                ReplClearOutput::ClearMemory => write!(f, "memory cleared"),
                ReplClearOutput::ClearVariable(v) => write!(f, "variable {} cleared", v),
            },
        }
    }
}
