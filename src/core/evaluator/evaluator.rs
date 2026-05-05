use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::parser::token::Token;
use crate::core::runtime_error::RuntimeError;

pub struct Evaluator {
    cursor: usize,
    tokens: Vec<Token>
}

impl Evaluator {
    pub fn new(tokens: Vec<Token>) -> Evaluator {
        Evaluator {
            cursor: 0,
            tokens
        }
    }

    pub fn evaluate(&mut self, rbp: u32) -> Result<f64, RuntimeError> {
        let token_idx = self.cursor;
        self.cursor += 1;
        let token = self.tokens[token_idx].clone();
        let mut left = match token.nud(self) {
            Some(left) => left,
            None => {
                return Err(EvaluationError::InvalidTokenPlace(format!("", )))
            }
        }

        while self.peek().lbp() > rbp {
            let next_idx = self.cursor;
            self.cursor += 1;

            let next_token = self.tokens[next_idx].clone();
            left = next_token.led(self, &left)
                .expect("Evaluator::evaluate [loop assign left] error");

        }
        left.as_f64().unwrap()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.cursor]
    }
}