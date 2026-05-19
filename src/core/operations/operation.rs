use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::evaluator::evaluator_result::Value;
use crate::core::parser::numeric::number_body::NumberBody;
use crate::core::parser::token::Token;
use crate::core::parser::token::Token::Number;

pub trait Operation: EvaluationRule {
    fn get_operand_count(&self) -> usize;
    fn get_sign(&self) -> &str;
    fn calc(&self, operands: &[f64]) -> Result<f64, EvaluationError>;

    fn get_result(&self, operands: &[f64]) -> Result<f64, EvaluationError> {
        if operands.len() != self.get_operand_count() {
            return Err(EvaluationError::ArityMismatch(
                self.get_operand_count(),
                operands.len(),
            ));
        }
        self.calc(operands)
    }

    fn default_led(&self, evaluator: &mut Evaluator, left: &Token) -> Result<Token, EvaluationError> {
        let right = match evaluator.evaluate(self.lbp()).map_err(|e| e.error)? {
            Value::Numeric(n) => n,
            other => return Err(EvaluationError::InvalidTokenPlace(other.to_string())),
        };
        let left_res = left.as_f64().ok_or(EvaluationError::MissingOperand)?;
        let res = self.get_result(&[left_res, right])?;
        Ok(Number(NumberBody { base: 10, raw: res.to_string(), decimal_value: res }))
    }
}