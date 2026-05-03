mod core;
use std::io::{Read, Write};
use colored::{ColoredString, Colorize};
use crate::core::evaluate_service::EvaluateService;

fn main() {
    let eval_service = EvaluateService::new();

    loop {
        let mut str = String::new();

        print!("> ");
        _ = std::io::stdout().flush();
        
        _ = std::io::stdin()
            .read_line(&mut str)
            .unwrap();

        let res = eval_service.evaluate(&str);

        match res {
            Ok(res) => println!("{} {}",
                                get_invite().green().bold(),
                                res.to_string().green().bold()),

            Err(error) => println!("{} {}",
                                   get_invite().red().bold(),
                                   error.red().bold())
        }

        println!();
    }
}

fn get_invite() -> String {
    ">>".to_string()
}