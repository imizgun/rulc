use crate::core::operations::operation::Operation;

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