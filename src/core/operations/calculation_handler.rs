pub trait CalculationHandler {
    fn get_operand_count(&self) -> u8;
    fn get_sign(&self) -> &str;
    fn calc(&self, operands: &[f64]) -> f64;

    fn get_result(&self, operands: &[f64]) -> f64 {
        if operands.len() != self.get_operand_count() as usize {
            panic!("operand count does not match");
        }

        self.calc(operands)
    }
}
