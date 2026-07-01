use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::operations::operation::Operation;

pub struct MultiplyOperation;

impl Operation for MultiplyOperation {
    fn get_operand_count(&self) -> usize {
        2
    }

    fn get_sign(&self) -> &str {
        "*"
    }

    fn calc(&self, operands: &[f64]) -> Result<f64, EvaluationError> {
        Ok(operands[0] * operands[1])
    }
}

impl EvaluationRule for MultiplyOperation {
    fn lbp(&self) -> u32 {
        20
    }
}
