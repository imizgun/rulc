use crate::core::parser::token::Token;

pub enum Statement {
    Expression(Vec<Token>),
    Assignment { name: String, tokens: Vec<Token> },
    FunctionDefinition { name: String, params: Vec<String>, body: Vec<Token> },
    DrawCommand {function_name: String, from_tokens: Vec<Token>, to_tokens: Vec<Token> }
}