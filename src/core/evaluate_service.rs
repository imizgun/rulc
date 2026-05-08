use crate::core::core_initializer::CoreInitializer;
use crate::core::evaluator::evaluator::Evaluator;
use crate::core::runtime_error::RuntimeError;

pub struct EvaluateService {
    core: CoreInitializer,
}

impl EvaluateService {
    pub fn new() -> EvaluateService {
        EvaluateService {
            core: CoreInitializer::new(),
        }
    }

    pub fn evaluate(&self, string: &str) -> Result<f64, RuntimeError> {
        let tokens = self.core.build_parser().parse_expression(string)?;
        let mut evaluator = Evaluator::new(tokens);
        Ok(evaluator.evaluate(0)?)
    }
}