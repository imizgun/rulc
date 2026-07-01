use std::fmt::{Display, Formatter};

pub enum ModeError {
    UnknownCommand,
    NoExpressionProvided,
}

impl Display for ModeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModeError::UnknownCommand => write!(
                f,
                "Unknown command. \nUsage: \n- rulc\n- rulc --tui\n- rulc --exec <expression>"
            ),
            ModeError::NoExpressionProvided => write!(f, "No expression provided for exec."),
        }
    }
}
