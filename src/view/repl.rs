use crate::core::evaluate_service::EvaluateService;
use crate::core::repl_output::{ReplClearOutput, ReplOutput};
use crate::core::runtime_error::RuntimeError;
use crate::view::viewable::Viewable;
use colored::Colorize;
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};
use rustyline::DefaultEditor;

pub struct ReplView;

impl ReplView {
    pub fn get_invite() -> String {
        ">>".to_string()
    }

    pub fn print_start_message() {
        println!(
            "\n{} {}\n",
            "Welcome to rulc!".green(),
            format!("Version: {}", env!("CARGO_PKG_VERSION")).bold()
        );
    }

    pub fn print_result(res: Result<ReplOutput, RuntimeError>) {
        match res {
            Ok(ReplOutput::IntersectionPoints { points }) if points.is_empty() => println!(
                "{} {}",
                ReplView::get_invite().green().bold(),
                "no intersection points found".green().bold()
            ),
            Ok(ReplOutput::IntersectionPoints { points }) => {
                println!(
                    "{} {}",
                    ReplView::get_invite().green().bold(),
                    format!("{} intersection point(s):", points.len())
                        .green()
                        .bold()
                );
                for (x, y) in &points {
                    println!("   ({x:.4}, {y:.4})");
                }
            }
            Ok(ReplOutput::Clear(ReplClearOutput::ClearHistory))
            | Ok(ReplOutput::Clear(ReplClearOutput::ClearAll)) => {
                crossterm::execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).ok();
            }
            Ok(ReplOutput::Help) => println!("{}", ReplOutput::Help),
            Ok(res) => println!(
                "{} {}",
                ReplView::get_invite().green().bold(),
                res.to_string().green().bold()
            ),

            Err(error) => {
                let lines = error.display_lines();
                println!(
                    "{} {}",
                    ReplView::get_invite().red().bold(),
                    format!("{}\n   {}", lines.formatted_tokens, lines.error)
                        .red()
                        .bold()
                )
            }
        }
    }
}

impl Viewable for ReplView {
    fn run(&self) {
        ReplView::print_start_message();

        let mut eval_service = EvaluateService::new();
        let mut rl = DefaultEditor::new().unwrap();

        loop {
            match rl.readline("> ") {
                Ok(line) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    rl.add_history_entry(trimmed).ok();
                    ReplView::print_result(eval_service.evaluate(trimmed));
                    println!();
                }
                Err(_) => break,
            }
        }
    }
}
