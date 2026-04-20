use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::parser::token::Token;

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(&mut self, tokens: Vec<Token>) -> f64 {
        let first_token = &tokens[0];
        let left = first_token.nud(self);
        0.0
    }
}