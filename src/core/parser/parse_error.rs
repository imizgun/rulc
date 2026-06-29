use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    UnknownOperator(String),
    InvalidNumber(String),
    UnmatchedParen,
    InvalidSyntax(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnknownOperator(op) => write!(f, "unknown operator: '{}'", op),
            ParseError::InvalidNumber(n) => write!(f, "invalid number: '{}'", n),
            ParseError::UnmatchedParen => write!(f, "unmatched parenthesis"),
            ParseError::InvalidSyntax(msg) => write!(f, "invalid syntax: {}", msg),
        }
    }
}