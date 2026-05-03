use crate::core::core_initializer::CoreInitializer;
use crate::core::evaluator::evaluator::Evaluator;

pub struct EvaluateService {
    core: CoreInitializer
}

impl EvaluateService {
    pub fn new() -> EvaluateService {
        EvaluateService {
            core: CoreInitializer::new()
        }
    }
    pub fn evaluate(&self, string: &str) -> Result<f64, String> {
        let parser = self.core.build_parser();
        
        let tokens = parser.parse_expression(string);

        match tokens {
            Ok(tokens) => {
                let mut evaluator = Evaluator::new(tokens);
                Ok(evaluator.evaluate(0))
            }
            Err(error) => Err(error)
        }
    }
}