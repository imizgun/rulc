use crate::core::evaluator::evaluator_result::Value;
use std::fmt::Display;

pub enum ReplOutput {
    Value(Value),
    Message(String),
    FuncPoints { points: Vec<(f64, f64)> },
    IntersectionPoints { points: Vec<(f64, f64)> },
    Clear(ReplClearOutput),
    Help,
}

pub const HELP_TEXT: &str =
    "rulc \u{2014} a simple calculator with variables, functions and plotting.

Examples:
  2 + 2 * 3                     arithmetic (+ - * / ^)
  (1 + 2) ^ 2                   parentheses
  x = 5                         assign a variable
  x + 1                         use it in an expression
  x += 2                        compound assignment
  f(x) = x^2 + 1                define a function
  f(3)                          call it
  draw f from 0 to 10           plot f(x) on [0, 10]
  intersect f g from 0 to 10    find intersection points of f and g on [0, 10]
  clear                         clear everything (plots, history, memory)
  clear plots                   clear plots only
  clear history                 clear REPL history
  clear memory                  clear user-defined variables/functions
  clear x                       clear a single variable/function
  help                          show this message

Builtin functions:
  sin, cos, tan, asin, acos, atan   trigonometric
  sqrt, abs, ceil, floor            general
  ln, log                          natural / base-10 logarithm

Builtin constants:
  pi, e";

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
            ReplOutput::Help => write!(f, "{}", HELP_TEXT),
        }
    }
}
