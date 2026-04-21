use std::future::pending;
use crate::core::core_initializer::CoreInitializer;
use crate::core::evaluator::evaluator::Evaluator;

pub struct EvaluateService;

impl EvaluateService {
    pub fn evaluate(string: &str) -> f64 {
        let init = CoreInitializer::new();
        let parser = init.build_parser();
        
        let tokens = parser.parse_expression(string);
        let mut evaluator = Evaluator::new(tokens);
        
        evaluator.evaluate(0)
    }
}