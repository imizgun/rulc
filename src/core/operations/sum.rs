use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::operations::operation::Operation;
use crate::core::parser::token::Token;
use crate::core::parser::parser::Parser;

pub struct SumOperation;
impl Operation for SumOperation {
    fn get_operand_count(&self) -> u8 { 2 }

    fn get_sign(&self) -> &str { "+" }
    
    fn calc(&self, operands: &[f64]) -> f64 {
        operands[0] + operands[1]
    }
}

impl EvaluationRule for SumOperation {
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