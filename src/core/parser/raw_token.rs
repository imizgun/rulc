use std::fmt::Debug;

#[derive(Debug)]
pub enum RawToken {
    Number(String),
    Identifier(String),
    Operator(String)
}