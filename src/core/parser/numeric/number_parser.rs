use crate::core::parser::numeric::number_body::NumberBody;

pub trait NumberParser {
    fn parse(str: &str) -> Option<NumberBody>;
    // fn get_base()
}