use crate::mode::Mode;
use crate::mode::Mode::{Repl, Tui};
use crate::mode_error::ModeError;
use crate::view::inline::InlineView;
use crate::view::repl::ReplView;
use crate::view::tui::TuiView;
use crate::view::viewable::Viewable;

mod core;
mod view;
mod mode;
pub mod mode_error;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let view: Box<dyn Viewable> = match parse_args(&args) { 
        Ok(v) => match v { 
            Repl => Box::new(ReplView),
            Tui => Box::new(TuiView),
            Mode::Inline(x) => Box::new(InlineView {expression: x}) 
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    
    view.run();
}

fn parse_args(args: &[String]) -> Result<Mode, ModeError> {
    if args.len() == 1 {
        return Ok(Mode::Repl);
    }
    match args[1].as_str() {
        "--tui" => Ok(Mode::Tui),
        "--exec" => {
            if args.len() != 3 {
                Err(ModeError::NoExpressionProvided)
            }
            else {
                Ok(Mode::Inline(args[2].to_string()))
            }
        },
        _ => Err(ModeError::UnknownCommand)
    }
}