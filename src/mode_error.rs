use std::fmt::{Display, Formatter};

pub const USAGE: &str = "Usage: \n\
- rulc                     start interactive REPL\n\
- rulc --tui, -t           start TUI mode\n\
- rulc --exec, -e <expr>   evaluate a single expression and exit\n\
- rulc --help, -h          show this message\n\
- <cmd> | rulc             pipe expressions (one per line) from stdin, print bare results";

pub enum ModeError {
    UnknownCommand,
    NoExpressionProvided,
}

impl Display for ModeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModeError::UnknownCommand => write!(f, "Unknown command. \n{}", USAGE),
            ModeError::NoExpressionProvided => write!(f, "No expression provided for exec."),
        }
    }
}
