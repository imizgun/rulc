use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum RawToken {
    Number(String),
    Identifier(String),
    Operator(String)
}