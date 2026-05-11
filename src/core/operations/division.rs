use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::operations::operation::Operation;

pub struct DivisionOperation;

impl Operation for DivisionOperation {
    fn get_operand_count(&self) -> u8 {
        2
    }

    fn get_sign(&self) -> &str {
        "/"
    }

    fn calc(&self, operands: &[f64]) -> f64 {
        let left = operands[0];
        let right = operands[1];

        left / right
    }
}

impl EvaluationRule for DivisionOperation {
    fn lbp(&self) -> u32 {
        20
    }
}