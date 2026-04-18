#[derive(PartialEq, Eq)]
pub enum ParserState {
    Idle,
    Number,
    Identifier,
    Operator
}