use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::operations::operation::Operation;
use crate::core::parser::numeric::number_body::NumberBody;
use crate::core::parser::token::Token;

pub struct MinusOperation;
impl Operation for MinusOperation {
    fn get_operand_count(&self) -> u8 { 2 }

    fn get_sign(&self) -> &str { "-" }

    fn calc(&self, operands: &[f64]) -> f64 {
        operands[0] - operands[1]
    }
}

impl EvaluationRule for MinusOperation {
    fn nud(&'_ self, evaluator: &mut Evaluator) -> Option<Token> {
        let right = evaluator.evaluate(100).ok()?;
        Some(Token::Number(NumberBody::from(-right)))
    }
    
    fn lbp(&self) -> u32 {
        10
    }
}