use std::fmt::Pointer;
use std::fmt::Debug;
use std::sync::Arc;
use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::operations::operation::Operation;
use crate::core::parser::numeric::number_body::NumberBody;

#[derive(Clone)]
pub enum Token {
    Number(NumberBody),
    Variable(String),
    Operation(Arc<dyn Operation>),
    Eof
}


impl Token {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Token::Number(body) => Some(body.decimal_value),
            _ => None
        }
    }
}
impl EvaluationRule for Token {
    fn nud(&self, evaluator: &mut Evaluator) -> Option<Token> {
        match self {
            Token::Number(n) => Some(Token::Number(n.clone())),
            Token::Operation(op) => op.nud(evaluator),
            Token::Variable(v) => todo!(),
            Token::Eof => None,
        }
    }

    fn led(&self, evaluator: &mut Evaluator, left: &Token) -> Option<Token> {
        match self {
            Token::Number(n) => None,
            Token::Variable(v) => todo!(),
            Token::Operation(op) => op.default_led(evaluator, left),
            Token::Eof => None,
        }
    }

    fn lbp(&self) -> u32 {
        match self {
            Token::Operation(op) => op.lbp(),
            _ => 0
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(b) => b.fmt(f),
            Token::Variable(v) => Debug::fmt(&v, f),
            Token::Operation(op) => op.fmt(f),
            Token::Eof => f.write_str("EOF")
        }
    }
}