use std::fmt::{Display, Formatter};

pub enum ParseError {
    UnknownOperator(String),
    UnknownIdentifier(String),
    InvalidNumber(String)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnknownOperator(operator) => write!(f, "unknown operator: '{}'", operator),
            ParseError::UnknownIdentifier(identifier) => write!(f, "unknown identifier: '{}'", identifier),
            ParseError::InvalidNumber(number) => write!(f, "invalid number: '{}'", number)
        }
    }
}