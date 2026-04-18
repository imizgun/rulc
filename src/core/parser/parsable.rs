use crate::core::parser::token::Token;

pub trait Parsable {
    fn parse(str: &str) -> Option<Token>;
}