use crate::core::parser::token::Token;

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(&mut self, tokens: Vec<Token>) -> f64 {
        let first_token = &tokens[0];
        let nud = first_token;
        0.0
    }
}