use std::io::LineWriter;

use crate::core::error_display::{ErrorContext, Located};
use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::parser::token::Token;

pub struct Evaluator {
    cursor: usize,
    tokens: Vec<Token>,
}

impl Evaluator {
    pub fn new(tokens: Vec<Token>) -> Evaluator {
        Evaluator { cursor: 0, tokens }
    }

    pub fn run(&mut self) -> Result<f64, Located<EvaluationError>> {
        let result = self.evaluate(0)?;

        if !matches!(self.tokens[self.cursor], Token::Eof) {
            return Err(Located::new(
                EvaluationError::MissingOperator,
                self.make_context(self.cursor - 1),
            ));
        }

        Ok(result)
    }

    pub fn evaluate(&mut self, rbp: u32) -> Result<f64, Located<EvaluationError>> {
        let token_idx = self.cursor;
        self.cursor += 1;
        let token = self.tokens[token_idx].clone();

        let mut left = token.nud(self).ok_or_else(|| {
            Located::new(
                EvaluationError::InvalidTokenPlace(token.to_string()),
                self.make_context(token_idx),
            )
        })?;

        while self.peek().lbp() > rbp {
            let next_idx = self.cursor;
            self.cursor += 1;
            let next_token = self.tokens[next_idx].clone();
            left = next_token.led(self, &left).ok_or_else(|| {
                Located::new(EvaluationError::MissingOperand, self.make_context(next_idx))
            })?
        }

        left.as_f64().ok_or_else(|| {
            Located::new(
                EvaluationError::MissingOperand,
                self.make_context(token_idx),
            )
        })
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.cursor]
    }

    fn make_context(&self, index: usize) -> ErrorContext {
        ErrorContext::new(
            self.tokens
                .iter()
                .filter(|t| !matches!(t, Token::Eof))
                .map(|t| t.to_string())
                .collect(),
            index,
        )
    }
}
