use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ErrorContext {
    pub tokens: Vec<String>,
    pub index: usize,
}

pub struct Located<E> {
    pub error: E,
    pub context: ErrorContext,
}

impl ErrorContext {
    pub fn new(tokens: Vec<String>, index: usize) -> Self {
        ErrorContext { tokens, index }
    }
}

impl<E> Located<E> {
    pub fn new(error: E, context: ErrorContext) -> Self {
        Located { error, context }
    }
}

impl<E: Display> Display for Located<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let offset = self.context.tokens[..self.context.index]
            .iter()
            .map(|t| t.len() + 1)
            .sum::<usize>();
        
        write!(
            f,
            "{}\n  ╰{}^ {}",
            self.context.tokens.join(" "),
            "—".repeat(offset),
            self.error
        )
    }
}

impl<E: Debug> Debug for Located<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Located({:?})", self.error)
    }
}