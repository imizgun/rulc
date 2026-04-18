use crate::core::parser::parsable::Parsable;
use crate::core::parser::token::Token;

pub struct NumberBody {
    raw: String,
    base: u8,
    decimal_value: f64
}

impl Parsable for NumberBody {
    fn parse(str: &str) -> Option<Token> {
        let res :f64 = str.trim()
            .parse()
            .expect("Provided string is not a number");

        Some(Token::Number(NumberBody {
            raw: str.to_string(),
            base: 10,
            decimal_value: res}))
    }
}