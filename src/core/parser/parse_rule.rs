use crate::core::parser::parser::Parser;
use crate::core::parser::token::Token;

pub trait ParseRule {
    // prefix handler (when token doesn't have anything on the left side)
    fn nud(&self, parser: &mut Parser) -> Option<Token> { None }

    // infix handler (when token is in the middle of expression)
    fn led(&self, parser: &mut Parser, left: Token) -> Option<Token> { None }

    // infix priority
    fn lbp(&self) -> u8 { 0 }
}