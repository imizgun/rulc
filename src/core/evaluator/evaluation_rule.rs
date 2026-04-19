use crate::core::parser::parser::Parser;
use crate::core::parser::token::Token;

pub trait EvaluationRule {
    // prefix handler (when token doesn't have anything on the left side)
    fn nud(&'_ self, parser: &mut Parser) -> Option<Token<'_>> { None }

    // infix handler (when token is in the middle of expression)
    fn led(&'_ self, parser: &mut Parser, left: Token) -> Option<Token<'_>> { None }

    // infix priority
    fn lbp(&self) -> u8 { 0 }
}