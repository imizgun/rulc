use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluation_rule::EvaluationRule;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::evaluator::evaluator_result::Value;
use crate::core::operations::operation::Operation;
use crate::core::parser::numeric::number_body::NumberBody;
use crate::core::parser::token::Token;


pub struct SumOperation;

impl Operation for SumOperation {
    fn get_operand_count(&self) -> usize { 2 }

    fn get_sign(&self) -> &str { "+" }

    fn calc(&self, operands: &[f64]) -> Result<f64, EvaluationError> {
        Ok(operands[0] + operands[1])
    }
}

impl EvaluationRule for SumOperation {
    fn nud(&self, evaluator: &mut Evaluator) -> Result<Token, EvaluationError> {
        let right = evaluator
            .evaluate(100)
            .map_err(|e| e.error)?;
        
        match right {
            Value::Numeric(num) => Ok(Token::Number(NumberBody::from(num))),
            other => Err(EvaluationError::InvalidTokenPlace(other.to_string())),
        }
    }

    fn lbp(&self) -> u32 {
        10
    }
}