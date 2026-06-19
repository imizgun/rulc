use crate::core::error_display::{ErrorContext, Located};
use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::evaluator::evaluator_result::Value;
use crate::core::parser::identifier_value::IdentifierValue;
use crate::core::parser::token::Token;
use crate::core::registries::identifiers_registry::IdentifiersRegistry;

pub struct Evaluator<'a> {
    cursor: usize,
    tokens: &'a [Token],
    pub identifier_registry: &'a IdentifiersRegistry
}

impl Evaluator<'_> {
    pub fn new<'a>(tokens: &'a [Token], identifiers_registry: &'a IdentifiersRegistry) -> Evaluator<'a> {
        Evaluator {
            cursor: 0,
            tokens: &tokens,
            identifier_registry: identifiers_registry }
    }

    pub fn run(&mut self) -> Result<Value, Located<EvaluationError>> {
        if self.tokens.len() - 1 == 1 {
            if let Token::Variable(name) = &self.tokens[0] {
                if let Some(IdentifierValue::Function(func)) = self.identifier_registry.get_identifier(name) {
                    return Ok(Value::Message(format!("function {}({:?}) = {:?}", name, func.parameters, func.function_body)));
                }
            }
        }

        let result = self.evaluate(0)?;

        if !matches!(self.tokens[self.cursor], Token::Eof) {
            return Err(Located::new(
                EvaluationError::MissingOperator,
                self.make_context(self.cursor - 1),
            ));
        }

        Ok(result)
    }

    pub fn evaluate(&mut self, rbp: u32) -> Result<Value, Located<EvaluationError>> {
        let token_idx = self.cursor;
        self.cursor += 1;
        let token = self.tokens[token_idx].clone();

        let mut left = token.nud(self).map_err(|e| {
            Located::new(e, self.make_context(token_idx))
        })?;

        while self.peek().lbp() > rbp {
            let next_idx = self.cursor;
            self.cursor += 1;
            let next_token = self.tokens[next_idx].clone();
            left = next_token.led(self, &left).map_err(|e| {
                Located::new(e, self.make_context(next_idx))
            })?
        }

        Ok(Value::Numeric(left.as_f64().ok_or_else(|| {
            Located::new(
                EvaluationError::MissingOperand,
                self.make_context(token_idx),
            )
        })?))
    }

    pub fn consume(&mut self) {
        self.cursor += 1;
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
