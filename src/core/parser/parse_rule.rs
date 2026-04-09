use crate::core::parser::parser::Parser;
use crate::core::parser::token::Token;

pub trait ParseRule {
    // prefix
    fn nud(&self, parser: &mut Parser) -> Option<Token> { None }

    // infix
    fn led(&self, parser: &mut Parser, left: Token) -> Option<Token> { None }

    // infix priority
    fn lbp(&self) -> u8 { 0 }
}