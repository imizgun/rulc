use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::operations::operation::Operation;

pub struct DivisionOperation;

impl Operation for DivisionOperation {
    fn get_operand_count(&self) -> usize {
        2
    }

    fn get_sign(&self) -> &str {
        "/"
    }

    fn calc(&self, operands: &[f64]) -> Result<f64, EvaluationError> {
        let left = operands[0];
        let right = operands[1];

        if right == 0.0 {
            return Err(EvaluationError::ArithmeticError(
                "division by zero".to_string(),
            ));
        }

        Ok(left / right)
    }
}

impl EvaluationRule for DivisionOperation {
    fn lbp(&self) -> u32 {
        20
    }
}
