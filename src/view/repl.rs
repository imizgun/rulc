use std::io::Write;
use colored::Colorize;
use crate::core::evaluate_service::EvaluateService;
use crate::core::repl_output::ReplOutput;
use crate::core::runtime_error::RuntimeError;
use crate::view::viewable::Viewable;

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
                    format!("{}\n   {}", lines.formatted_tokens, lines.error).red().bold()
                )
            },
        }
    }
}

impl Viewable for ReplView {
    fn run(&self) {
        ReplView::print_start_message();

        let mut eval_service = EvaluateService::new();

        loop {
            let mut str = String::new();

            print!("> ");
            _ = std::io::stdout().flush();

            _ = std::io::stdin().read_line(&mut str).unwrap();

            if str.trim().is_empty() {
                continue;
            }

            let res = eval_service.evaluate(&str);

            ReplView::print_result(res);

            println!();
        }
    }
}