#[derive(PartialEq, Eq)]
pub enum LexerState {
    Idle,
    Number,
    Identifier,
    Operator
}