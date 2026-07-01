use crate::core::evaluator::evaluation_error::EvaluationError;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::parser::token::Token;

pub trait EvaluationRule {
    // prefix handler (when token doesn't have anything on the left side)
    fn nud(&'_ self, _evaluator: &mut Evaluator) -> Result<Token, EvaluationError> {
        Err(EvaluationError::MissingOperand)
    }

    // infix handler (when token is in the middle of expression)
    fn led(&'_ self, _evaluator: &mut Evaluator, _left: &Token) -> Result<Token, EvaluationError> {
        Err(EvaluationError::MissingOperand)
    }

    // infix priority
    fn lbp(&self) -> u32 {
        0
    }
}
