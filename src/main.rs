mod core;
use std::io::Write;
use colored::Colorize;
use crate::core::evaluate_service::EvaluateService;

fn main() {
    let mut eval_service = EvaluateService::new();

    loop {
        let mut str = String::new();

        print!("> ");
        _ = std::io::stdout().flush();
        
        _ = std::io::stdin()
            .read_line(&mut str)
            .unwrap();

        if str.trim().is_empty() {
            continue
        }

        let res = eval_service.evaluate(&str);

        match res {
            Ok(res) => println!("{} {}",
                                get_invite().green().bold(),
                                res.to_string().green().bold()),

            Err(error) => println!("{} {}",
                                   get_invite().red().bold(),
                                   error.to_string().red().bold())
        }

        println!();
    }
}

fn get_invite() -> String {
    ">>".to_string()
}