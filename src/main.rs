mod core;
use std::io::Write;
use crate::core::evaluate_service::EvaluateService;

fn main() {
    let mut str = String::new();

    loop {
        print!("> ");
        _ = std::io::stdout().flush();
        
        _ = std::io::stdin()
            .read_line(&mut str)
            .unwrap();

        println!(">> {}", EvaluateService::evaluate(&str));
        println!();
    }
}