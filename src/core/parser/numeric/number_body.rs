use crate::core::parser::parsable::Parsable;

#[derive(Debug, Clone)]
pub struct NumberBody {
    pub raw: String,
    pub base: u8,
    pub decimal_value: f64
}

impl Parsable<NumberBody> for NumberBody {
    fn parse(str: &str) -> Option<NumberBody> {
        match str.trim().parse() {
            Ok(num) => Some(NumberBody {
                raw: str.to_string(),
                base: 10,
                decimal_value: num}),
            Err(err) => None
        }
    }
}