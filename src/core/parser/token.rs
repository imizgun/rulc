use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::operations::operation::Operation;
use crate::core::parser::numeric::number_body::NumberBody;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use crate::core::evaluator::evaluator_result::Value;
use crate::core::parser::identifier_value::IdentifierValue;

#[derive(Clone)]
pub enum Token {
    Number(NumberBody),
    Variable(String),
    Operation(Arc<dyn Operation>),
    OpenParen,
    CloseParen,
    Eof,
}

impl Token {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Token::Number(body) => Some(body.decimal_value),
            _ => None,
        }
    }
}

impl EvaluationRule for Token {
    fn nud(&self, evaluator: &mut Evaluator) -> Result<Token, EvaluationError> {
        match self {
            Token::Number(n) => Ok(Token::Number(n.clone())),
            Token::Operation(op) => op.nud(evaluator),
            Token::OpenParen => {
                let result = evaluator.evaluate(0).map_err(|e| e.error)?;
                evaluator.consume();
                match result {
                    Value::Numeric(n) => Ok(Token::Number(NumberBody::from(n))),
                    other => Err(EvaluationError::InvalidTokenPlace(other.to_string())),
                }
            }
            Token::Variable(name) => match evaluator.identifier_registry.get_identifier(name) {
                Some(IdentifierValue::Number(num)) => Ok(Token::Number(NumberBody::from(num))),

                Some(IdentifierValue::Function(func)) => Ok(Token::Number(
                    NumberBody::from(func.value()?))),

                None => Err(EvaluationError::UnknownIdentifier(name.clone())),
            },
            Token::CloseParen | Token::Eof => Err(EvaluationError::InvalidTokenPlace(self.to_string())),
        }
    }

    fn led(&self, evaluator: &mut Evaluator, left: &Token) -> Result<Token, EvaluationError> {
        match self {
            Token::Operation(op) => op.default_led(evaluator, left),
            _ => Err(EvaluationError::MissingOperand),
        }
    }

    fn lbp(&self) -> u32 {
        match self {
            Token::Operation(op) => op.lbp(),
            _ => 0,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(b) => write!(f, "{}", b.raw),
            Token::Variable(v) => write!(f, "{}", v),
            Token::Operation(op) => write!(f, "{}", op.get_sign()),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
