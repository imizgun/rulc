use std::fmt::Pointer;
use std::fmt::Debug;
use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::operations::operation::Operation;
use crate::core::parser::numeric::number_body::NumberBody;
use crate::core::parser::parser::Parser;

pub enum Token<'a> {
    Number(NumberBody),
    Variable(String),
    Operation(&'a Box<dyn Operation>)
}

impl EvaluationRule for Token<'_> {
    fn nud(&self, parser: &mut Parser) -> Option<Token> {
        todo!()
    }

    fn led(&self, parser: &mut Parser, left: Token) -> Option<Token> {
        todo!()
    }

    fn lbp(&self) -> u8 {
        todo!()
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(b) => b.fmt(f),
            Token::Variable(v) => v.fmt(f),
            Token::Operation(op) => op.fmt(f),
        }
    }
}