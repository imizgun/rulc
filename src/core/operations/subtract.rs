use crate::core::operations::calculation_handler::CalculationHandler;

pub struct SubtractOperation;
impl CalculationHandler for SubtractOperation {
    fn get_operand_count(&self) -> u8 { 2 }

    fn get_sign(&self) -> &str { "-" }

    fn calc(&self, operands: &[f64]) -> f64 {
        operands[0] - operands[1]
    }
}