use crate::mode::Mode;
use crate::mode::Mode::{Repl, Tui};
use crate::mode_error::{ModeError, USAGE};
use crate::view::inline::InlineView;
use crate::view::pipe::PipeView;
use crate::view::repl::ReplView;
use crate::view::tui::TuiView;
use crate::view::viewable::Viewable;
use std::io::IsTerminal;

mod core;
mod mode;
pub mod mode_error;
mod view;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mode = match parse_args(&args) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Mode::Help = mode {
        println!("{}", USAGE);
        return;
    }

    let view: Box<dyn Viewable> = match mode {
        Repl if !std::io::stdin().is_terminal() => Box::new(PipeView),
        Repl => Box::new(ReplView),
        Tui => Box::new(TuiView),
        Mode::Inline(x) => Box::new(InlineView { expression: x }),
        Mode::Help => unreachable!(),
    };

    view.run();
}

fn parse_args(args: &[String]) -> Result<Mode, ModeError> {
    if args.len() == 1 {
        return Ok(Repl);
    }
    match args[1].as_str() {
        "--tui" | "-t" => Ok(Tui),
        "--exec" | "-e" => {
            if args.len() != 3 {
                Err(ModeError::NoExpressionProvided)
            } else {
                Ok(Mode::Inline(args[2].to_string()))
            }
        }
        "--help" | "-h" => Ok(Mode::Help),
        _ => Err(ModeError::UnknownCommand),
    }
}
