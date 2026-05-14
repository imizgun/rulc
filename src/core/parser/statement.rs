use crate::core::parser::token::Token;

pub enum Statement {
    Expression(Vec<Token>),
    Assignment { name: String, tokens: Vec<Token> },
}