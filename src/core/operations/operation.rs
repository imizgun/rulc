use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::parser::numeric::number_body::NumberBody;
use crate::core::parser::token::Token;
use crate::core::parser::token::Token::Number;

pub trait Operation: EvaluationRule {
    fn get_operand_count(&self) -> u8;
    fn get_sign(&self) -> &str;
    fn calc(&self, operands: &[f64]) -> f64;

    fn get_result(&self, operands: &[f64]) -> f64 {
        if operands.len() != self.get_operand_count() as usize {
            panic!("operand count does not match");
        }

        self.calc(operands)
    }

    fn default_led(&self, evaluator: &mut Evaluator, left: &Token) -> Option<Token> {
        let right = evaluator.evaluate(self.lbp()).ok()?;
        let left_res = left.as_f64()?;
        let res = self.get_result(&[left_res, right]);
        Some(Number(NumberBody { base: 10, raw: res.to_string(), decimal_value: res }))
    }
}