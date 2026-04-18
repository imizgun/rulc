use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::operations::operation::Operation;
use crate::core::parser::number_body::NumberBody;
use crate::core::parser::parsable::Parsable;
use crate::core::parser::parser::Parser;

pub enum Token {
    Number(NumberBody),
    Variable(String),
    Operation(Box<dyn Operation>)
}

impl EvaluationRule for Token {
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