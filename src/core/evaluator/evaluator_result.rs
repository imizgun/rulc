#[derive(Debug)]
pub enum EvaluatorResult {
    None,
    Boolean(bool),
    Numeric(f64),
    Message(String)
}