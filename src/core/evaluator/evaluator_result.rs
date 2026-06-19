use std::fmt::Display;

#[derive(Debug)]
pub enum Value {
    Numeric(f64),
    Boolean(bool),
    Message(String)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Numeric(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Message(m) => write!(f, "{}", m)
        }
    }
}