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

pub struct LocatedErrorDisplay {
    pub formatted_tokens: String,
    pub error: String
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

    pub fn unlocated(error: E) -> Self {
        Located { error, context: ErrorContext::new(vec![], 0) }
    }
}

impl<E: Display> Located<E> {
    pub fn get_lines(&self) -> LocatedErrorDisplay {
        if self.context.tokens.is_empty() {
            return LocatedErrorDisplay {
                formatted_tokens: String::new(),
                error: self.error.to_string(),
            };
        }

        let offset = self.context.tokens[..self.context.index]
            .iter()
            .map(|t| t.len() + 1)
            .sum::<usize>();

        LocatedErrorDisplay {
            formatted_tokens: self.context.tokens.join(" "),
            error: format!("╰{}^ {}", "—".repeat(offset.saturating_sub(1)), self.error),
        }
    }
}

impl<E: Display> Display for Located<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines = self.get_lines();
        write!(f, "{}\n{}", lines.formatted_tokens, lines.error)
    }
}

impl<E: Debug> Debug for Located<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Located({:?})", self.error)
    }
}