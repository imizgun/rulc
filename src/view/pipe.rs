use crate::core::evaluate_service::EvaluateService;
use crate::view::viewable::Viewable;
use std::io::BufRead;

pub struct PipeView;

impl Viewable for PipeView {
    fn run(&self) {
        let mut service = EvaluateService::new();
        let stdin = std::io::stdin();
        let mut had_error = false;

        for line in stdin.lock().lines() {
            let Ok(line) = line else { break };
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            match service.evaluate(trimmed) {
                Ok(output) => println!("{}", output),
                Err(error) => {
                    eprintln!("{}", error);
                    had_error = true;
                }
            }
        }

        if had_error {
            std::process::exit(1);
        }
    }
}
