use crate::core::operations::calculation_handler::CalculationHandler;
use crate::core::parser::parse_rule::ParseRule;
use crate::core::parser::token::Token;
use crate::core::parser::parser::Parser;

pub struct SumOperation;
impl CalculationHandler for SumOperation {
    fn get_operand_count(&self) -> u8 { 2 }

    fn get_sign(&self) -> &str { "+" }
    
    fn calc(&self, operands: &[f64]) -> f64 {
        operands[0] + operands[1]
    }
}

impl ParseRule for SumOperation {
    fn nud(&self, parser: &mut Parser) -> Option<Token> {
        self.get_result()
    }

    fn led(&self, parser: &mut Parser, left: Token) -> Option<Token> {
        todo!()
    }

    fn lbp(&self) -> u8 {
        todo!()
    }
}