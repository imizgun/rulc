mod core;
use std::io::{Read, Write};
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
            Ok(res) => println!(">> {}", res),
            Err(error) => println!(">> {}", error)
        }

        println!();
    }
}