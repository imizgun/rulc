mod core;
use crate::core::parser::parser::Parser;

fn main() {
    let mut str = String::new();

    _ = std::io::stdin().read_line(&mut str).unwrap();

    Parser::parse_expression(&str);
}