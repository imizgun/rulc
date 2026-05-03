use std::fmt::{Debug, Display, Formatter};

pub enum RawToken {
    Number(String),
    Identifier(String),
    Operator(String),
    Eof
}

impl Display for RawToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RawToken::Number(s) => write!(f, "{}", s),
            RawToken::Identifier(s) => write!(f, "{}", s),
            RawToken::Operator(s) => write!(f, "{}", s),
            RawToken::Eof => write!(f, "EOF"),
        }
    }
}

impl Debug for RawToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}