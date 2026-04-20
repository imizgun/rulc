use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::operations::operation::Operation;
use crate::core::parser::token::Token;

pub struct MultiplyOperation;

impl Operation for MultiplyOperation {
    fn get_operand_count(&self) -> u8 {
        2
    }

    fn get_sign(&self) -> &str {
        "*"
    }

    fn calc(&self, operands: &[f64]) -> f64 {
        operands[0] * operands[1]
    }
}

impl EvaluationRule for MultiplyOperation {
    fn led(&'_ self, evaluator: &mut Evaluator, left: &Token) -> Option<Token> {
        todo!()
    }

    fn lbp(&self) -> u32 {
        20
    }
}